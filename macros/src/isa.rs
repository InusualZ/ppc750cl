use proc_macro::{quote, Delimiter, Group, Ident, Literal, Span, TokenStream, TokenTree};
use std::iter::FromIterator;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{And, EqEq};
use syn::{LitInt, LitStr, Token};

struct Opcodes {
    opcodes: Punctuated<Opcode, Token![;]>,
}

impl Parse for Opcodes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            opcodes: Punctuated::parse_terminated(input)?,
        })
    }
}

#[derive(Debug)]
struct Opcode {
    name: String,
    variant_name: String,
    mask: u32,
    bits: u32,
}

impl Parse for Opcode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<LitStr>()?;
        input.parse::<And>()?;
        let mask = input.parse::<LitInt>()?;
        input.parse::<EqEq>()?;
        let bits = input.parse::<LitInt>()?;
        Ok(Self {
            name: name.value(),
            variant_name: opcode_to_variant_name(&name)?,
            mask: hex_from_lit_int(&mask)?,
            bits: hex_from_lit_int(&bits)?,
        })
    }
}

fn hex_from_lit_int(int: &LitInt) -> syn::Result<u32> {
    let str = int.to_string();
    let hex = match str.strip_prefix("0x") {
        None => return Err(syn::Error::new(int.span(), "not a hex integer")),
        Some(x) => x,
    };
    u32::from_str_radix(hex, 16).map_err(|e| syn::Error::new(int.span(), e))
}

fn opcode_to_variant_name(opcode: &LitStr) -> syn::Result<String> {
    let mut s = String::new();
    let opcode_value = opcode.value();
    let mut chars = opcode_value.chars();
    loop {
        // Make first char uppercase.
        let c = match chars.next() {
            None => return Ok(s),
            Some(c) => c,
        };
        match c {
            'a'..='z' => s.push(c.to_ascii_uppercase()),
            _ => return Err(syn::Error::new(opcode.span(), "invalid opcode name")),
        }
        loop {
            let c = match chars.next() {
                None => return Ok(s),
                Some(c) => c,
            };
            match c {
                '0'..='9' | 'a'..='z' => s.push(c),
                '_' => break,
                '.' => {
                    s.push('_');
                    break;
                }
                _ => {
                    return Err(syn::Error::new(
                        opcode.span(),
                        "invalid character in opcode name",
                    ))
                }
            }
        }
    }
}

fn gen_is_valid_fn(tokens: &mut Vec<TokenTree>, opcodes: &Opcodes) {
    tokens.extend(quote!(pub fn is_valid(self, code: u32) -> bool));

    let mut parts = Vec::<TokenTree>::new();
    parts.extend(quote!(match self));

    let mut match_parts = Vec::<TokenTree>::new();
    match_parts.extend(quote!(Opcode::Illegal => false,));

    for opcode in &opcodes.opcodes {
        match_parts.extend(quote!(Opcode::));
        match_parts.push(TokenTree::Ident(Ident::new(
            &opcode.variant_name,
            Span::call_site(),
        )));
        match_parts.extend(quote!(=> code &));
        match_parts.push(TokenTree::Literal(Literal::u32_suffixed(opcode.mask)));
        match_parts.extend(quote!(==));
        match_parts.push(TokenTree::Literal(Literal::u32_suffixed(opcode.bits)));
        match_parts.extend(quote!(,));
    }
    let match_body = Group::new(Delimiter::Brace, TokenStream::from_iter(match_parts));
    parts.push(TokenTree::Group(match_body));
    let body = Group::new(Delimiter::Brace, TokenStream::from_iter(parts));
    tokens.push(TokenTree::Group(body));
}

fn gen_mnemonic_fn(tokens: &mut Vec<TokenTree>, opcodes: &Opcodes) {
    tokens.extend(quote!(pub fn mnemonic(self) -> &'static str));

    let mut parts = Vec::<TokenTree>::new();
    parts.extend(quote!(match self));

    let mut match_parts = Vec::<TokenTree>::new();
    match_parts.extend(quote!(Opcode::Illegal => "<illegal>",));
    for opcode in &opcodes.opcodes {
        match_parts.extend(quote!(Opcode::));
        match_parts.push(TokenTree::Ident(Ident::new(
            &opcode.variant_name,
            Span::call_site(),
        )));
        match_parts.extend(quote!(=>));
        match_parts.push(TokenTree::Literal(Literal::string(&opcode.name)));
        match_parts.extend(quote!(,));
    }

    let match_body = Group::new(Delimiter::Brace, TokenStream::from_iter(match_parts));
    parts.push(TokenTree::Group(match_body));

    let body = Group::new(Delimiter::Brace, TokenStream::from_iter(parts));
    tokens.push(TokenTree::Group(body));
}

pub(crate) fn isa(input: TokenStream) -> TokenStream {
    let opcodes = syn::parse_macro_input!(input as Opcodes);

    // Assemble root stream.
    let mut root = Vec::<TokenTree>::new();

    // Define enum derives and header.
    let derives = quote!(#[derive(Debug, Copy, Clone, Eq, PartialEq)]);
    root.append(&mut derives.into_iter().collect());
    let enum_header = quote!(pub enum Opcode);
    root.append(&mut enum_header.into_iter().collect());

    // Create entries.
    // First entry is going to be the illegal entry.
    let mut enum_entries = Vec::<TokenTree>::new();
    enum_entries.append(&mut (quote!(Illegal = -1,).into_iter().collect()));
    // Append the actual opcodes.
    for opcode in &opcodes.opcodes {
        enum_entries.push(TokenTree::Ident(Ident::new(
            &opcode.variant_name,
            Span::call_site(),
        )));
        enum_entries.extend(quote!(,));
    }

    // Create body.
    let enum_body = Group::new(Delimiter::Brace, TokenStream::from_iter(enum_entries));
    root.push(TokenTree::Group(enum_body));

    // Default implementation.
    let opcode_default = quote! {
        impl Default for Opcode {
            fn default() -> Self {
                Opcode::Illegal
            }
        }
    };
    root.append(&mut opcode_default.into_iter().collect());

    // impl Opcode block.
    let impl_opcode_header = quote!(impl Opcode);
    root.append(&mut impl_opcode_header.into_iter().collect());
    let mut impl_opcode_body_parts = Vec::<TokenTree>::new();
    gen_is_valid_fn(&mut impl_opcode_body_parts, &opcodes);
    gen_mnemonic_fn(&mut impl_opcode_body_parts, &opcodes);
    let impl_opcode_body = Group::new(
        Delimiter::Brace,
        TokenStream::from_iter(impl_opcode_body_parts),
    );
    root.push(TokenTree::Group(impl_opcode_body));

    // impl ToString block.
    let to_string_trait_impl = quote! {
        impl std::string::ToString for Opcode {
            fn to_string(&self) -> String {
                let mnemonic = self.mnemonic();
                mnemonic.to_owned()
            }
        }
    };
    root.append(&mut to_string_trait_impl.into_iter().collect());

    TokenStream::from_iter(root)
}

#[proc_macro]
#[cfg(test)]
mod tests {
    use super::*;

    fn _opcode_to_variant_name(input: &str) -> String {
        opcode_to_variant_name(&LitStr::new(input, proc_macro2::Span::call_site())).unwrap()
    }

    #[test]
    fn test_opcode_to_variant_name() {
        assert_eq!(_opcode_to_variant_name("lol_lel."), "LolLel_");
        assert_eq!(_opcode_to_variant_name("ps_nmsub"), "PsNmsub");
    }
}