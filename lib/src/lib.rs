#![allow(clippy::unusual_byte_groupings)]

use std::io::Write;
use std::ops::Range;

use num_traits::AsPrimitive;

use ppc750cl_macros::isa;

isa! {
    "add" & 0xfc0007fe == 0x7c000214;
    "addc" & 0xfc0007fe == 0x7c00002a;
    "adde" & 0xfc0007fe == 0x7c000114;
    "addi" & 0xfc000000 == 0x38000000;
    "addic" & 0xfc000000 == 0x30000000;
    "addic." & 0xfc000000 == 0x34000000;
    "addis" & 0xfc000000 == 0x3c000000;
    "addme" & 0xfc00fbfe == 0x7c0001d4;
    "addze" & 0xfc00fbfe == 0x7c000194;
    "and" & 0xfc0007fe == 0x7c000038;
    "andc" & 0xfc0007fe == 0x7c000078;
    "andi." & 0xfc000000 == 0x70000000;
    "andis." & 0xfc000000 == 0x74000000;
    "b" & 0xfc000000 == 0x48000000;
    "bc" & 0xfc000000 == 0x40000000;
    "bcctr" & 0xfc00ffff == 0x4c000210;
    "bclr" & 0xfc00fffe == 0x4c000020;
    "cmp" & 0xfc4007ff == 0x7c000000;
    "cmpi" & 0xfc400000 == 0x2c000000;
    "cmpl" & 0xfc4007ff == 0x7c000040;
    "cmpli" & 0xfc400000 == 0x28000000;
    "cntlzw" & 0xfc00fffe == 0x7c000034;
    "crand" & 0xfc0007ff == 0x4c000202;
    "crandc" & 0xfc0007ff == 0x4c000102;
    "creqv" & 0xfc0007ff == 0x4c000242;
    "crnand" & 0xfc0007ff == 0x4c0001c2;
    "crnor" & 0xfc0007ff == 0x4c000042;
    "cror" & 0xfc0007ff == 0x4c000382;
    "crorc" & 0xfc0007ff == 0x4c000342;
    "crxor" & 0xfc0007ff == 0x4c000182;
    "dcbf" & 0xffe007ff == 0x7c0000ac;
    "dcbi" & 0xffe007ff == 0x7c0003ac;
    "dcbst" & 0xffe007ff == 0x7c00006c;
    "dcbt" & 0xffe007ff == 0x7c00022c;
    "dcbtst" & 0xffe007ff == 0x7c0001ec;
    "dcbz" & 0xffe007ff == 0x7c0007ec;
    "dcbz_l" & 0xffe007ff == 0x100007ec;
    "divw" & 0xfc0003fe == 0x7c0003d6;
    "divwu" & 0xfc0003fe == 0x7c000396;
    "eciwx" & 0xfc0003ff == 0x7c00026c;
    "ecowx" & 0xfc0003ff == 0x7c00036c;
    "eieio" & 0xffffffff == 0x7c0006ac;
    "eqv" & 0xfc0003fe == 0x7c000238;
    "extsb" & 0xfc00fffe == 0x7c000774;
    "extsh" & 0xfc00fffe == 0x7c000734;
    "fabs" & 0xfc1f07fe == 0xfc000734;
    "fadd" & 0xfc0007fe == 0xfc00002a;
    "fadds" & 0xfc0007fe == 0xec00002a;
    "fcmpo" & 0xfc6007ff == 0xfc000040;
    "fcmpu" & 0xfc6007ff == 0xfc000000;
    "fctiw" & 0xfc1f07fe == 0xfc00001c;
    "fctiwz" & 0xfc1f07fe == 0xfc00001e;
    "fdiv" & 0xfc0007fe == 0xfc000024;
    "fdivs" & 0xfc0007fe == 0xec000024;
    "fmadd" & 0xfc00003e == 0xfc00003a;
    "fmadds" & 0xfc00003e == 0xec00003a;
    "fmr" & 0xfc1f07fe == 0xfc000090;
    "fmsub" & 0xfc00003e == 0xfc000038;
    "fmsubs" & 0xfc00003e == 0xec000038;
    "fmul" & 0xfc00f83e == 0xfc000032;
    "fmuls" & 0xfc00f83e == 0xec000032;
    "fnabs" & 0xfc1f07fe == 0xfc000110;
    "fneg" & 0xfc1f07fe == 0xfc000050;
    "fnmadd" & 0xfc00003e == 0xfc00003e;
    "fnmadds" & 0xfc00003e == 0xec00003e;
    "fnmsub" & 0xfc00003e == 0xfc00003c;
    "fnmsubs" & 0xfc00003e == 0xec00003c;
    "fres" & 0xfc1f07fe == 0xec000030;
    "frsp" & 0xfc1f07fe == 0xfc000018;
    "frsqrte" & 0xfc1f07fe == 0xfc000034;
    "fsel" & 0xfc00003e == 0xfc00002e;
    "fsub" & 0xfc0007fe == 0xfc000028;
    "fsubs" & 0xfc0007fe == 0xec000028;
    "icbi" & 0xffe007ff == 0x7c0007ac;
    "isync" & 0xffffffff == 0x4c00012c;
    "lbz" & 0xfc000000 == 0x88000000;
    "lbzu" & 0xfc000000 == 0x8c000000;
    "lbzux" & 0xfc0007ff == 0x7c0000ee;
    "lbzx" & 0xfc0007ff == 0x7c0000ae;
    "lfd" & 0xfc000000 == 0xc8000000;
    "lfdu" & 0xfc000000 == 0xcc000000;
    "lfdux" & 0xfc0007ff == 0x7c0004ee;
    "lfdx" & 0xfc0007ff == 0x7c00045e;
    "lfs" & 0xfc000000 == 0xc0000000;
    "lfsu" & 0xfc000000 == 0xc4000000;
    "lfsux" & 0xfc0007ff == 0x7c00046e;
    "lfsx" & 0xfc0007ff == 0x7c00042e;
    "lha" & 0xfc000000 == 0xa8000000;
    "lhau" & 0xfc000000 == 0xac000000;
    "lhaux" & 0xfc0007ff == 0x7c0002ee;
    "lhax" & 0xfc0007ff == 0x7c0002ae;
    "lhbrx" & 0xfc0007ff == 0x7c00062c;
    "lhz" & 0xfc000000 == 0xa0000000;
    "lhzu" & 0xfc000000 == 0xa4000000;
    "lhzux" & 0xfc0007ff == 0x7c00026e;
    "lhzx" & 0xfc0007ff == 0x7c00022e;
    "lmw" & 0xfc000000 == 0xb8000000;
    "lswi" & 0xfc0007ff == 0x7c0004aa;
    "lswx" & 0xfc0007ff == 0x7c00042a;
    "lwarx" & 0xfc0007ff == 0x7c000028;
    "lwbrx" & 0xfc0007ff == 0x7c00042c;
    "lwz" & 0xfc000000 == 0x80000000;
    "lwzu" & 0xfc000000 == 0x84000000;
    "lwzux" & 0xfc0007ff == 0x7c00006e;
    "lwzx" & 0xfc0007ff == 0x7c00002e;
    "mcrf" & 0xfc300fff == 0x4c000000;
    "mcrfs" & 0xfc30ffff == 0xfc000080;
    "mcrxr" & 0xfc30ffff == 0x7c000400;
    "mfcr" & 0xfc1fffff == 0x7c000026;
    "mffs" & 0xfc1ffffe == 0x7c00048e;
    "mfmsr" & 0xfc1fffff == 0x7c0000a6;
    "mfspr" & 0xfc0007ff == 0x7c0002a6;
    "mfsr" & 0xfc10ffff == 0x7c0004a6;
    "mfsrin" & 0xfc1f07ff == 0x7c000526;
    "mftb" & 0xfc0007ff == 0x7c0002e6;
    "mtcrf" & 0xfc100fff == 0x7c000120;
    "mtfsb0" & 0xfc1ffffe == 0xfc00008c;
    "mtfsb1" & 0xfc1ffffe == 0xfc00004c;
    "mtfsf" & 0xfe0107fe == 0xfc00058e;
    "mtfsfi" & 0xfc7f0ffe == 0xfc00010c;
    "mtmsr" & 0xfc1fffff == 0x7c000124;
    "mtspr" & 0xfc0007ff == 0x7c0003a6;
    "mtsr" & 0xfc10ffff == 0x7c0001a4;
    "mtsrin" & 0xfc1f07ff == 0x7c0001e4;
    "mulhw" & 0xfc0007fe == 0x7c000096;
    "mulhwu" & 0xfc0007fe == 0x7c000016;
    "mulli" & 0xfc000000 == 0x1c000000;
    "mullw" & 0xfc0003fe == 0x7c0001d6;
    "nand" & 0xfc0007fe == 0x7c0003b8;
    "neg" & 0xfc00fffe == 0x7c0000d0;
    "nor" & 0xfc0007fe == 0x7c0000f8;
    "or" & 0xfc0007fe == 0x7c000378;
    "orc" & 0xfc0007fe == 0x7c000338;
    "ori" & 0xfc000000 == 0x60000000;
    "oris" & 0xfc000000 == 0x64000000;
    "psq_l" & 0xfc000000 == 0xe0000000;
    "psq_lu" & 0xfc000000 == 0xe4000000;
    "psq_lux" & 0xfc00007f == 0x1000004c;
    "psq_lx" & 0xfc00007f == 0x1000000c;
    "psq_st" & 0xfc000000 == 0xf0000000;
    "psq_stu" & 0xfc000000 == 0xf4000000;
    "psq_stux" & 0xfc00007f == 0x1000004e;
    "psq_stx" & 0xfc00007f == 0x1000000e;
    "ps_abs" & 0xfc1f07fe == 0x10000210;
    "ps_add" & 0xfc0007fe == 0x1000002a;
    "ps_cmpo0" & 0xfc6007ff == 0x10000040;
    "ps_cmpo1" & 0xfc6007ff == 0x100000c0;
    "ps_cmpu0" & 0xfc6007ff == 0x10000000;
    "ps_cmpu1" & 0xfc6007ff == 0x10000080;
    "ps_div" & 0xfc0007fe == 0x10000024;
    "ps_madd" & 0xfc00003e == 0x1000003a;
    "ps_madds0" & 0xfc00003e == 0x1000001c;
    "ps_madds1" & 0xfc00003e == 0x1000001e;
    "ps_merge00" & 0xfc0007fe == 0x10000420;
    "ps_merge01" & 0xfc0007fe == 0x10000460;
    "ps_merge10" & 0xfc0007fe == 0x100004a0;
    "ps_merge11" & 0xfc0007fe == 0x100004e0;
    "ps_mr" & 0xfc1f07fe == 0x10000090;
    "ps_msub" & 0xfc00003e == 0x10000038;
    "ps_mul" & 0xfc00f83e == 0x10000032;
    "ps_muls0" & 0xfc00f83e == 0x10000018;
    "ps_muls1" & 0xfc00f83e == 0x1000001a;
    "ps_nabs" & 0xfc1f07fe == 0x10000110;
    "ps_neg" & 0xfc1f07fe == 0x10000050;
    "ps_nmadd" & 0xfc00003e == 0x1000003e;
    "ps_nmsub" & 0xfc00003e == 0x1000003c;
    "ps_res" & 0xfc1f07fe == 0x10000030;
    "ps_rsqrte" & 0xfc1f07fe == 0x10000034;
    "ps_sel" & 0xfc00003e == 0x1000002e;
    "ps_sub" & 0xfc0007fe == 0x10000028;
    "ps_sum0" & 0xfc00003e == 0x10000014;
    "ps_sum1" & 0xfc00003e == 0x10000016;
    "rfi" & 0xfffff801 == 0x4c000000;
    "rlwimi" & 0xfc000000 == 0x50000000;
    "rlwinm" & 0xfc000000 == 0x54000000;
    "rlwnm" & 0xfc000000 == 0x5c000000;
    "sc" & 0xffffffff == 0x44000002;
    "slw" & 0xfc0007fe == 0x7c000030;
    "sraw" & 0xfc0007fe == 0x7c000630;
    "srawi" & 0xfc0007fe == 0x7c000670;
    "srw" & 0xfc0007fe == 0x7c000430;
    "stb" & 0xfc000000 == 0x98000000;
    "stbu" & 0xfc000000 == 0x9c000000;
    "stbux" & 0xfc0003ff == 0x7c0001ee;
    "stbx" & 0xfc0003ff == 0x7c0001ae;
    "stfd" & 0xfc000000 == 0xd8000000;
    "stfdu" & 0xfc000000 == 0xdc000000;
    "stfdux" & 0xfc0003ff == 0x7c0005ee;
    "stfdx" & 0xfc0003ff == 0x7c0005ae;
    "stfiwx" & 0xfc0007ff == 0x7c0007ae;
    "stfs" & 0xfc000000 == 0xd0000000;
    "stfsu" & 0xfc000000 == 0xd4000000;
    "stfsux" & 0xfc0003ff == 0x7c00056e;
    "stfsx" & 0xfc0003ff == 0x7c00052e;
    "sth" & 0xfc000000 == 0xb0000000;
    "sthbrx" & 0xfc0007ff == 0x7c00072c;
    "sthu" & 0xfc000000 == 0xb4000000;
    "sthux" & 0xfc0003ff == 0x7c00036e;
    "sthx" & 0xfc0003ff == 0x7c00032e;
    "stmw" & 0xfc000000 == 0xbc000000;
    "stswi" & 0xfc0007ff == 0x7c0005aa;
    "stswx" & 0xfc0007ff == 0x7c00052a;
    "stw" & 0xfc000000 == 0x90000000;
    "stwbrx" & 0xfc0007ff == 0x7c00052c;
    "stwcx." & 0xfc0007ff == 0x7c00012d;
    "stwu" & 0xfc000000 == 0x94000000;
    "stwux" & 0xfc0003ff == 0x7c00016e;
    "stwx" & 0xfc0003ff == 0x7c00012e;
    "subf" & 0xfc0003fe == 0x7c000050;
    "subfc" & 0xfc0003fe == 0x7c000010;
    "subfe" & 0xfc0003fe == 0x7c000110;
    "subfic" & 0xfc000000 == 0x20000000;
    "subfme" & 0xfc00fbfe == 0x7c0001d0;
    "subfze" & 0xfc00fbfe == 0x7c000190;
    "sync" & 0xffffffff == 0x7c0004ac;
    "tlbie" & 0xffff07ff == 0x7c000264;
    "tlbsync" & 0xffffffff == 0x7c00046c;
    "tw" & 0xfc0007ff == 0x7c000008;
    "twi" & 0xfc000000 == 0xc000000;
    "xor" & 0xfc0007fe == 0x7c000278;
    "xori" & 0xfc000000 == 0x68000000;
    "xoris" & 0xfc000000 == 0x6c000000;
}

#[derive(Default, Clone)]
pub struct Ins {
    pub code: u32,
    pub op: Opcode,
}

#[inline(always)]
fn bit(x: u32, idx: usize) -> u8 {
    ((x >> (32 - idx - 1)) & 1) as u8
}

#[inline(always)]
fn bits<F>(x: u32, range: Range<usize>) -> F
where
    F: 'static + std::marker::Copy,
    u32: AsPrimitive<F>,
{
    let masked: u32 = (x >> (32 - range.end)) & ((1 << range.len()) - 1);
    masked.as_()
}

macro_rules! disasm_unreachable {
    ($msg:expr $(,)?) => {{
        panic!(
            "internal error: entered unreachable code disassembling instruction 0x{:08x}",
            $msg
        )
    }};
}

macro_rules! ins_bit {
    ($func:ident, $idx:expr) => {
        fn $func(&self) -> u8 {
            bit(self.code, $idx)
        }
    };
}

macro_rules! ins_field {
    ($func:ident, $return_type:tt, $range:expr) => {
        fn $func(&self) -> $return_type {
            debug_assert!(
                ($range).len() / 8 <= (std::mem::size_of::<$return_type>()),
                "{:?} does not fit in {}",
                $range,
                stringify!($return_type)
            );
            bits(self.code, $range)
        }
    };
}

impl Ins {
    fn new(code: u32, op: Opcode) -> Self {
        Ins { code, op }
    }

    fn illegal() -> Self {
        Default::default()
    }

    //ins_bit!(w, 21);
    ins_bit!(rc, 31);
    ins_bit!(aa, 30);
    ins_bit!(lk, 31);
    ins_bit!(l, 10);
    ins_bit!(oe, 21);
    ins_bit!(w, 16);

    // Registers
    ins_field!(s, u8, 6..11);
    ins_field!(d, u8, 6..11);
    ins_field!(a, u8, 11..16);
    ins_field!(b, u8, 16..21);
    ins_field!(c, u8, 21..26);
    // Condition registers
    ins_field!(crb_d, u8, 6..11);
    ins_field!(crb_a, u8, 11..16);
    ins_field!(crb_b, u8, 16..21);

    ins_field!(crm, u8, 12..20);
    ins_field!(sr, u8, 12..16);
    fn spr(&self) -> u16 {
        bits::<u16>(self.code, 11..16) | (bits::<u16>(self.code, 16..21) << 5)
    }
    ins_field!(fm, u16, 7..15);
    ins_field!(crf_d, u8, 6..9);
    ins_field!(crf_s, u8, 11..14);
    ins_field!(simm, i16, 16..32);
    ins_field!(uimm, u16, 16..32);
    ins_field!(bo, u8, 6..11);
    ins_field!(bi, u8, 11..16);
    ins_field!(sh, u8, 16..21);
    ins_field!(mb, u8, 21..26);
    ins_field!(me, u8, 26..31);
    //ins_field!(bd, u16, 16..30);
    ins_field!(li, u32, 6..30);
    ins_field!(to, u8, 6..11);
    // Paired-single fields.
    ins_field!(ps_l, u8, 17..20);
    ins_field!(ps_d, u16, 20..32);

    pub fn disasm(x: u32) -> Self {
        let family = bits(x, 0..6);
        let mut ins = match family {
            0b000011 => Ins::new(x, Opcode::Twi),
            0b000100 => Self::disasm_cl_ext(x),
            0b000111..=0b001111 => Self::disasm_basic1(x),
            0b010000 => Ins::new(x, Opcode::Bc),
            0b010001 => Ins::new(x, Opcode::Sc),
            0b010010 => Ins::new(x, Opcode::B),
            0b010011 => Self::disasm_010011(x),
            0b010100..=0b011101 => Self::disasm_basic2(x),
            0b011111 => Self::disasm_011111(x),
            0b100000..=0b110111 => Self::disasm_basic3(x),
            0b111000..=0b111001 => Self::disasm_psq(x),
            0b111011 => Self::disasm_111011(x),
            0b111100..=0b111101 => Self::disasm_psq(x),
            0b111111 => Self::disasm_111111(x),
            _ => Self::illegal(),
        };
        if !ins.op.is_valid(x) {
            ins.op = Opcode::Illegal;
        }
        ins
    }

    fn disasm_cl_ext(x: u32) -> Self {
        let op = match bits(x, 26..31) {
            0b00000 => match bits(x, 26..31) {
                0b00000 => Opcode::PsCmpu0,
                0b00001 => Opcode::PsCmpo0,
                0b00010 => Opcode::PsCmpu1,
                0b00011 => Opcode::PsCmpo1,
                _ => Opcode::Illegal,
            },
            0b00110 => {
                if bit(x, 25) == 0 {
                    Opcode::PsqLx
                } else {
                    Opcode::PsqLux
                }
            }
            0b00111 => {
                if bit(x, 25) == 0 {
                    Opcode::PsqStx
                } else {
                    Opcode::PsqStux
                }
            }
            0b01010 => Opcode::PsSum0,
            0b01011 => Opcode::PsSum1,
            0b01110 => Opcode::PsMadds0,
            0b01111 => Opcode::PsMadds1,
            0b10111 => Opcode::PsSel,
            0b11100 => Opcode::PsMsub,
            0b11101 => Opcode::PsMadd,
            0b11110 => Opcode::PsNmsub,
            0b11111 => Opcode::PsNmadd,
            0b01100 => Opcode::PsMuls0,
            0b01101 => Opcode::PsMuls1,
            0b11001 => Opcode::PsMul,
            0b10010 => Opcode::PsDiv,
            0b10100 => Opcode::PsSub,
            0b10101 => Opcode::PsAdd,
            0b11000 => Opcode::PsRes,
            0b11010 => Opcode::PsRsqrte,
            0b01000 => match bits(x, 26..31) {
                0b00001 => Opcode::PsNeg,
                0b00010 => Opcode::PsMr,
                0b00100 => Opcode::PsNabs,
                0b01000 => Opcode::PsAbs,
                _ => Opcode::Illegal,
            },
            0b10000 => match bits(x, 26..31) {
                0b10000 => Opcode::PsMerge00,
                0b10001 => Opcode::PsMerge01,
                0b10010 => Opcode::PsMerge10,
                0b10011 => Opcode::PsMerge11,
                _ => Opcode::Illegal,
            },
            0b10110 => Opcode::DcbzL,
            // Unknown paired-singles key.
            _ => Opcode::Illegal,
        };
        Ins::new(x, op)
    }

    fn disasm_basic1(x: u32) -> Self {
        let op = match bits(x, 0..6) {
            0b000111 => Opcode::Mulli,
            0b001000 => Opcode::Subfic,
            0b001010 => Opcode::Cmpli,
            0b001011 => Opcode::Cmpi,
            0b001100 => Opcode::Addic,
            0b001101 => Opcode::Addic_,
            0b001110 => Opcode::Addi,
            0b001111 => Opcode::Addis,
            _ => Opcode::Illegal,
        };
        Ins::new(x, op)
    }

    fn disasm_010011(x: u32) -> Self {
        let op = match bits(x, 21..27) {
            0b000000 => Opcode::Mcrf,
            0b000001 => Opcode::Bclr,
            0b100001 => Opcode::Bcctr,
            0b000011 => Opcode::Rfi,
            0b001001 => Opcode::Isync,
            0b000010 => Opcode::Crnor,
            0b001000 => Opcode::Crandc,
            0b001100 => Opcode::Crxor,
            0b001110 => Opcode::Crnand,
            0b010000 => Opcode::Crand,
            0b010010 => Opcode::Creqv,
            0b011010 => Opcode::Crorc,
            0b011100 => Opcode::Cror,
            _ => Opcode::Illegal,
        };
        Ins::new(x, op)
    }

    fn disasm_basic2(x: u32) -> Self {
        let op = match bits(x, 0..6) {
            0b10100 => Opcode::Rlwimi,
            0b10101 => Opcode::Rlwinm,
            0b10111 => Opcode::Rlwnm,
            0b11000 => Opcode::Ori,
            0b11001 => Opcode::Oris,
            0b11010 => Opcode::Xori,
            0b11011 => Opcode::Xoris,
            0b11100 => Opcode::Andi_,
            0b11101 => Opcode::Andis_,
            _ => Opcode::Illegal,
        };
        Ins::new(x, op)
    }

    fn disasm_011111(x: u32) -> Self {
        let op = match bits::<u32>(x, 21..31) {
            0b00_0000_0000 => Opcode::Cmp,
            0b00_0010_0000 => Opcode::Cmpl,
            0b00_0000_0100 => Opcode::Tw,
            0b00_0000_1000 => Opcode::Subfc,
            0b00_0000_1010 => Opcode::Addc,
            0b00_0000_1011 => Opcode::Mulhwu,
            0b00_0001_0011 => Opcode::Mfcr,
            0b00_0001_0100 => Opcode::Lwarx,
            0b00_0001_0111 => Opcode::Lwzx,
            0b00_0001_1000 => Opcode::Slw,
            0b00_0001_1010 => Opcode::Cntlzw,
            0b00_0001_1100 => Opcode::And,
            0b00_0010_1000 => Opcode::Subf,
            0b00_0011_0110 => Opcode::Dcbst,
            0b00_0011_0111 => Opcode::Lwzux,
            0b00_0011_1100 => Opcode::Andc,
            0b00_0100_1101 => Opcode::Mulhw,
            0b00_0101_0011 => Opcode::Mfmsr,
            0b00_0101_0110 => Opcode::Dcbf,
            0b00_0101_0111 => Opcode::Lbzx,
            0b00_0110_1000 => Opcode::Neg,
            0b00_0111_0111 => Opcode::Lbzux,
            0b00_0111_1100 => Opcode::Nor,
            0b00_1000_1000 => Opcode::Subfe,
            0b00_1000_1010 => Opcode::Adde,
            0b00_1001_0000 => Opcode::Mtcrf,
            0b00_1001_0010 => Opcode::Mtmsr,
            0b00_1001_0110 => Opcode::Stwcx_,
            0b00_1001_0111 => Opcode::Stwx,
            0b00_1011_0111 => Opcode::Stwux,
            0b00_1100_1000 => Opcode::Subfze,
            0b00_1100_1010 => Opcode::Addze,
            0b00_1101_0010 => Opcode::Mtsr,
            0b00_1101_0111 => Opcode::Stbx,
            0b00_1110_1000 => Opcode::Subfme,
            0b00_1110_1010 => Opcode::Addme,
            0b00_1110_1011 => Opcode::Mullw,
            0b00_1111_0010 => Opcode::Mtsrin,
            0b00_1111_0110 => Opcode::Dcbtst,
            0b00_1111_0111 => Opcode::Stbux,
            0b01_0000_1010 => Opcode::Add,
            0b01_0000_0110 => Opcode::Dcbt,
            0b01_0000_0111 => Opcode::Lhzx,
            0b01_0001_1100 => Opcode::Eqv,
            0b01_0011_0010 => Opcode::Tlbie,
            0b01_0011_0110 => Opcode::Eciwx,
            0b01_0011_0111 => Opcode::Lhzux,
            0b01_0011_1100 => Opcode::Xor,
            0b01_0101_0011 => Opcode::Mfspr,
            0b01_0101_0111 => Opcode::Lhax,
            0b01_0111_0011 => Opcode::Mftb,
            0b01_0111_0111 => Opcode::Lhaux,
            0b01_1001_0111 => Opcode::Sthx,
            0b01_1001_1100 => Opcode::Orc,
            0b01_1011_0110 => Opcode::Ecowx,
            0b01_1011_0111 => Opcode::Sthux,
            0b01_1011_1100 => Opcode::Or,
            0b01_1100_1011 => Opcode::Divwu,
            0b01_1101_0011 => Opcode::Mtspr,
            0b01_1101_0110 => Opcode::Dcbi,
            0b01_1101_1100 => Opcode::Nand,
            0b01_1111_1011 => Opcode::Divw,
            0b10_0000_0000 => Opcode::Mcrxr,
            0b10_0001_0101 => Opcode::Lswx,
            0b10_0001_0110 => Opcode::Lwbrx,
            0b10_0001_0111 => Opcode::Lfsx,
            0b10_0001_1000 => Opcode::Srw,
            0b10_0011_0110 => Opcode::Tlbsync,
            0b10_0011_0111 => Opcode::Lfsux,
            0b10_0101_0011 => Opcode::Mfsr,
            0b10_0101_0101 => Opcode::Lswi,
            0b10_0101_0110 => Opcode::Sync,
            0b10_0101_0111 => Opcode::Lfdx,
            0b10_0111_0111 => Opcode::Lfdux,
            0b10_1001_0011 => Opcode::Mfsrin,
            0b10_1001_0101 => Opcode::Stswx,
            0b10_1001_0110 => Opcode::Stwbrx,
            0b10_1001_0111 => Opcode::Stfsx,
            0b10_1011_0111 => Opcode::Stfsux,
            0b10_1101_0101 => Opcode::Stswi,
            0b10_1101_0111 => Opcode::Stfdx,
            0b10_1111_0111 => Opcode::Stfdux,
            0b11_0001_0110 => Opcode::Lhbrx,
            0b11_0001_1000 => Opcode::Sraw,
            0b11_0011_1000 => Opcode::Srawi,
            0b11_0101_0110 => Opcode::Eieio,
            0b11_1001_0110 => Opcode::Sthbrx,
            0b11_1001_1010 => Opcode::Extsh,
            0b11_1011_1010 => Opcode::Extsb,
            0b11_1101_0110 => Opcode::Icbi,
            0b11_1101_0111 => Opcode::Stfiwx,
            0b11_1111_0110 => Opcode::Dcbz,
            _ => Opcode::Illegal,
        };
        Ins::new(x, op)
    }

    fn disasm_basic3(x: u32) -> Self {
        let op = match bits(x, 0..6) {
            0b100000 => Opcode::Lwz,
            0b100001 => Opcode::Lwzu,
            0b100010 => Opcode::Lbz,
            0b100011 => Opcode::Lbzu,
            0b100100 => Opcode::Stw,
            0b100101 => Opcode::Stwu,
            0b100110 => Opcode::Stb,
            0b100111 => Opcode::Stbu,
            0b101000 => Opcode::Lhz,
            0b101001 => Opcode::Lhzu,
            0b101010 => Opcode::Lha,
            0b101011 => Opcode::Lhau,
            0b101100 => Opcode::Sth,
            0b101101 => Opcode::Sthu,
            0b101110 => Opcode::Lmw,
            0b101111 => Opcode::Stmw,
            0b110000 => Opcode::Lfs,
            0b110001 => Opcode::Lfsu,
            0b110010 => Opcode::Lfd,
            0b110011 => Opcode::Lfdu,
            0b110100 => Opcode::Stfs,
            0b110101 => Opcode::Stfsu,
            0b110110 => Opcode::Stfd,
            0b110111 => Opcode::Stfdu,
            _ => disasm_unreachable!(x),
        };
        Ins::new(x, op)
    }

    fn disasm_psq(x: u32) -> Self {
        let op = match bits(x, 0..6) {
            0b111000 => Opcode::PsqL,
            0b111001 => Opcode::PsqLu,
            0b111100 => Opcode::PsqSt,
            0b111101 => Opcode::PsqStu,
            _ => disasm_unreachable!(x),
        };
        Ins::new(x, op)
    }

    fn disasm_111011(x: u32) -> Self {
        let op = match bits(x, 26..31) {
            0b10010 => Opcode::Fdivs,
            0b10100 => Opcode::Fsubs,
            0b10101 => Opcode::Fadds,
            0b11000 => Opcode::Fres,
            0b11001 => Opcode::Fmuls,
            0b11100 => Opcode::Fmsubs,
            0b11101 => Opcode::Fmadds,
            0b11110 => Opcode::Fnmsubs,
            0b11111 => Opcode::Fnmadds,
            _ => Opcode::Illegal,
        };
        Ins::new(x, op)
    }

    fn disasm_111111(x: u32) -> Self {
        let op = match bits::<u32>(x, 26..31) {
            0b00000 => match bits(x, 26..31) {
                0b00 => Opcode::Fcmpu,
                0b01 => Opcode::Fcmpo,
                0b10 => Opcode::Mcrfs,
                _ => Opcode::Illegal,
            },
            0b00110 => match bits(x, 26..31) {
                0b001 => Opcode::Mtfsb1,
                0b010 => Opcode::Mtfsb0,
                0b100 => Opcode::Mtfsfi,
                _ => Opcode::Illegal,
            },
            0b00111 => match bits(x, 26..31) {
                0b10010 => Opcode::Mffs,
                0b10110 => Opcode::Mtfsf,
                _ => Opcode::Illegal,
            },
            0b01000 => match bits(x, 26..31) {
                0b0001 => Opcode::Fneg,
                0b0010 => Opcode::Fabs,
                0b0100 => Opcode::Fnabs,
                0b1000 => Opcode::Fmr,
                _ => Opcode::Illegal,
            },
            0b01100 => Opcode::Frsp,
            0b01110 => Opcode::Fctiw,
            0b01111 => Opcode::Fctiwz,
            0b10010 => Opcode::Fdiv,
            0b10100 => Opcode::Fsub,
            0b10101 => Opcode::Fadd,
            0b10111 => Opcode::Fsel,
            0b11001 => Opcode::Fmul,
            0b11010 => Opcode::Frsqrte,
            0b11100 => Opcode::Fmsub,
            0b11101 => Opcode::Fmadd,
            0b11110 => Opcode::Fnmsub,
            0b11111 => Opcode::Fnmadd,
            _ => Opcode::Illegal,
        };
        Ins::new(x, op)
    }

    fn write_string_form_reg123<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Eciwx => "eciwx",
            Opcode::Ecowx => "ecowx",
            Opcode::Lhaux => "lhaux",
            Opcode::Lhax => "lhax",
            Opcode::Lbzux => "lbzux",
            Opcode::Lbzx => "lbzx",
            Opcode::Lhbrx => "lhbrx",
            Opcode::Lhzux => "lhzux",
            Opcode::Lhzx => "lhzx",
            Opcode::Lswx => "lswx",
            Opcode::Lwarx => "lwarx",
            Opcode::Lwbrx => "lwbrx",
            Opcode::Lwzx => "lwzx",
            Opcode::Lwzux => "lwzux",
            Opcode::Stbux => "stbux",
            Opcode::Stbx => "stbx",
            Opcode::Sthux => "sthux",
            Opcode::Sthx => "sthx",
            Opcode::Sthbrx => "sthbrx",
            Opcode::Stswx => "stswx",
            Opcode::Stwbrx => "stwbrx",
            Opcode::Stwcx_ => "stwcx.",
            Opcode::Stwx => "stwx",
            Opcode::Stwux => "stwux",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, r{}, r{}", name, self.d(), self.a(), self.b())
    }

    fn write_string_form_reg123_rc<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::And => "and",
            Opcode::Andc => "andc",
            Opcode::Mulhw => "mulhw",
            Opcode::Mulhwu => "mulhwu",
            Opcode::Xor => "xor",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} r{}, r{}, r{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.b()
        )
    }

    fn write_string_form_reg123_oe_rc<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = match (self.oe() != 0, self.rc() != 0) {
            (false, false) => "",
            (false, true) => ".",
            (true, false) => "o",
            (true, true) => "o.",
        };
        let name = match self.op {
            Opcode::Add => "add",
            Opcode::Addc => "addc",
            Opcode::Adde => "adde",
            Opcode::Divw => "divw",
            Opcode::Divwu => "divwu",
            Opcode::Mullw => "mullw",
            Opcode::Subf => "subf",
            Opcode::Subfc => "subfc",
            Opcode::Subfe => "subfe",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} r{}, r{}, r{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.b()
        )
    }

    fn write_string_noargs<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Eieio => "eieio",
            Opcode::Isync => "isync",
            Opcode::Rfi => "rfi",
            Opcode::Sc => "sc",
            Opcode::Sync => "sync",
            Opcode::Tlbsync => "tlbsync",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{}", name)
    }

    fn write_string_form_reg12_simm<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Addi => "addi",
            Opcode::Addic => "addic",
            Opcode::Addic_ => "addic.",
            Opcode::Addis => "addis",
            Opcode::Mulli => "mulli",
            Opcode::Subfic => "subfic",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} r{}, r{}, {}",
            name,
            self.d(),
            self.a(),
            self.simm()
        )
    }

    fn write_string_form_reg12_uimm<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Andi_ => "andi.",
            Opcode::Andis_ => "andis.",
            Opcode::Ori => "ori",
            Opcode::Oris => "oris",
            Opcode::Xori => "xori",
            Opcode::Xoris => "xoris",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} r{}, r{}, {}",
            name,
            self.d(),
            self.a(),
            self.uimm()
        )
    }

    fn write_string_form_reg12_offset<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Lha => "lha",
            Opcode::Lhau => "lhau",
            Opcode::Lbz => "lbz",
            Opcode::Lbzu => "lbzu",
            Opcode::Lhz => "lhz",
            Opcode::Lhzu => "lhzu",
            Opcode::Lmw => "lmw",
            Opcode::Lwz => "lwz",
            Opcode::Lwzu => "lwzu",
            Opcode::Stb => "stb",
            Opcode::Stbu => "stbu",
            Opcode::Sth => "sth",
            Opcode::Sthu => "sthu",
            Opcode::Stmw => "stmw",
            Opcode::Stw => "stw",
            Opcode::Stwu => "stwu",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} r{}, {}(r{})",
            name,
            self.d(),
            self.simm(),
            self.a()
        )
    }

    fn write_string_form_fr1_reg2_offset<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Lfd => "lfd",
            Opcode::Lfdu => "lfdu",
            Opcode::Lfs => "lfs",
            Opcode::Lfsu => "lfsu",
            Opcode::Stfd => "stfd",
            Opcode::Stfdu => "stfdu",
            Opcode::Stfs => "stfs",
            Opcode::Stfsu => "stfsu",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} fr{}, {}(r{})",
            name,
            self.d(),
            self.simm(),
            self.a()
        )
    }

    fn write_string_form_fr1_reg23<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Lfdux => "lfdux",
            Opcode::Lfdx => "lfdx",
            Opcode::Lfsux => "lfsux",
            Opcode::Lfsx => "lfsx",
            Opcode::Stfdux => "stfdux",
            Opcode::Stfdx => "stfdx",
            Opcode::Stfiwx => "stfiwx",
            Opcode::Stfsux => "stfsux",
            Opcode::Stfsx => "stfsx",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} fr{}, r{}, r{}", name, self.d(), self.a(), self.b())
    }

    fn write_string_mtfsf<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mtfsf => "mtfsf",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} {}, fr{}", name, self.fm(), self.b())
    }

    fn write_string_mtfsfi<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mtfsfi => "mtfsfi",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} crf{}, {}",
            name,
            self.crf_d(),
            bits::<u8>(self.code, 16..20)
        )
    }

    fn write_string_form_reg1<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mfcr => "mfcr",
            Opcode::Mfmsr => "mfmsr",
            Opcode::Mtmsr => "mtmsr",
            Opcode::Tlbie => "tblie",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}", name, self.d())
    }

    fn write_string_form_reg12_oe_rc<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = match (self.oe() != 0, self.rc() != 0) {
            (false, false) => "",
            (false, true) => ".",
            (true, false) => "o",
            (true, true) => "o.",
        };
        let name = match self.op {
            Opcode::Addme => "addme",
            Opcode::Addze => "addze",
            Opcode::Neg => "neg",
            Opcode::Subfme => "subfme",
            Opcode::Subfze => "subfze",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{}{} r{}, r{}", name, name_suffix, self.d(), self.a())
    }

    fn write_string_form_reg13<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mfsrin => "mfsrin",
            Opcode::Mtsrin => "mtsrin",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, r{}", name, self.d(), self.b())
    }

    fn write_string_form_reg21_rc<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::Cntlzw => "cntlzw",
            Opcode::Extsb => "extsb",
            Opcode::Extsh => "extsh",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{}{} r{}, r{}", name, name_suffix, self.a(), self.s())
    }

    fn write_string_form_fr1<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mffs => match self.rc() != 0 {
                false => "mffs",
                true => "mffs.",
            },
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} fr{}", name, self.d())
    }

    fn write_string_form_fr13<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::Fabs => "fabs",
            Opcode::Fnabs => "fnabs",
            Opcode::Fmr => "fmr",
            Opcode::Fneg => "fneg",
            Opcode::Fres => "fres",
            Opcode::Frsp => "frsp",
            Opcode::Frsqrte => "frsqrte",
            Opcode::PsAbs => "ps_abs",
            Opcode::PsMr => "ps_mr",
            Opcode::PsNabs => "ps_nabs",
            Opcode::PsNeg => "ps_neg",
            Opcode::PsRes => "ps_res",
            Opcode::PsRsqrte => "ps_rsqrte",
            Opcode::PsSum0 => "ps_sum0",
            Opcode::PsSum1 => "ps_sum1",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} fr{}, fr{}",
            name,
            name_suffix,
            self.d(),
            self.b()
        )
    }

    fn write_string_form_fr123<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::Fadd => "fadd",
            Opcode::Fadds => "fadds",
            Opcode::Fdiv => "fdiv",
            Opcode::Fdivs => "fdivs",
            Opcode::Fsub => "fsub",
            Opcode::Fsubs => "fsubs",
            Opcode::PsAdd => "ps_add",
            Opcode::PsDiv => "ps_div",
            Opcode::PsMerge00 => "ps_merge00",
            Opcode::PsMerge01 => "ps_merge01",
            Opcode::PsMerge10 => "ps_merge10",
            Opcode::PsMerge11 => "ps_merge11",
            Opcode::PsSub => "ps_sub",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} fr{}, fr{}, fr{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.b()
        )
    }

    fn write_string_form_fr1243<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::Fmadd => "fmadd",
            Opcode::Fmadds => "fmadds",
            Opcode::Fmsub => "fmsub",
            Opcode::Fmsubs => "fmsubs",
            Opcode::Fnmadd => "fnmadd",
            Opcode::Fnmadds => "fnmadds",
            Opcode::Fnmsub => "fnmsub",
            Opcode::Fnmsubs => "fnmsubs",
            Opcode::Fsel => "fsel",
            Opcode::PsMadd => "ps_madd",
            Opcode::PsMadds0 => "ps_madds0",
            Opcode::PsMadds1 => "ps_madds1",
            Opcode::PsMsub => "ps_msub",
            Opcode::PsNmadd => "ps_nmadd",
            Opcode::PsNmsub => "ps_nmsub",
            Opcode::PsSel => "ps_sel",
            Opcode::PsSum0 => "ps_sum0",
            Opcode::PsSum1 => "ps_sum1",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} fr{}, fr{}, fr{}, fr{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.c(),
            self.b()
        )
    }

    fn write_string_form_fr124<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::Fmul => "fmul",
            Opcode::Fmuls => "fmuls",
            Opcode::PsMul => "ps_mul",
            Opcode::PsMuls0 => "ps_muls0",
            Opcode::PsMuls1 => "ps_muls1",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} fr{}, fr{}, fr{}",
            name,
            name_suffix,
            self.d(),
            self.a(),
            self.c()
        )
    }

    fn write_string_form_condreg1_fr23<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Fcmpo => "fcmpo",
            Opcode::Fcmpu => "fcmpu",
            Opcode::PsCmpo0 => "ps_cmpo0",
            Opcode::PsCmpo1 => "ps_cmpo1",
            Opcode::PsCmpu0 => "ps_cmpu0",
            Opcode::PsCmpu1 => "ps_cmpu1",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} crf{}, fr{}, fr{}",
            name,
            self.crf_d(),
            self.a(),
            self.b()
        )
    }

    fn write_string_form_condreg1_fr13_rc<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::Fctiw => "fctiw",
            Opcode::Fctiwz => "fctiwz",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} crf{}, fr{}, fr{}",
            name,
            name_suffix,
            self.crf_d(),
            self.d(),
            self.b()
        )
    }

    fn write_string_b<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match (self.aa() != 0, self.lk() != 0) {
            (false, false) => "b",
            (false, true) => "bl",
            (true, false) => "ba",
            (true, true) => "bla",
        };
        // TODO absolute address
        write!(out, "{} 0x{:x}", name, self.li())
    }

    fn write_string_bc<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match (self.aa() != 0, self.lk() != 0) {
            (false, false) => "bc",
            (false, true) => "bcl",
            (true, false) => "bca",
            (true, true) => "bcla",
        };
        // TODO absolute address
        write!(
            out,
            "{} 0x{:x}, 0x{:x}, 0x{:x}",
            name,
            self.bo(),
            self.bi(),
            self.li()
        )
    }

    fn write_string_branch_cond_to_reg<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Bcctr => match self.lk() != 0 {
                false => "bcctr",
                true => "bcctrl",
            },
            Opcode::Bclr => match self.lk() != 0 {
                false => match (self.bo(), self.bi()) {
                    (0b01100, 0b00000) => return write!(out, "bltlr"),
                    (0b00100, 0b01010) => return write!(out, "bnelr cr2"),
                    (0b10000, 0b00000) => return write!(out, "bdnzlr"),
                    (0b10100, 0b00000) => return write!(out, "blr"),
                    _ => "bclr",
                },
                true => "bclrl",
            },
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} 0x{:x}, 0x{:x}", name, self.bo(), self.bi())
    }

    fn write_string_cmp<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Cmp => "cmp",
            Opcode::Cmpl => "cmpl",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} crf{}, {}, r{}, r{}",
            name,
            self.crf_d(),
            self.l() as u8,
            self.a(),
            self.b()
        )
    }

    fn write_string_cmp_simm<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = "cmpi";
        write!(
            out,
            "{} crf{}, {}, r{}, {}",
            name,
            self.crf_d(),
            self.l() as u8,
            self.a(),
            self.simm()
        )
    }

    fn write_string_cmp_uimm<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = "cmpli";
        write!(
            out,
            "{} crf{}, {}, r{}, {}",
            name,
            self.crf_d(),
            self.l() as u8,
            self.a(),
            self.uimm()
        )
    }

    fn write_string_form_condreg1<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mcrxr => "mcrxr",
            Opcode::Mtfsb0 => match self.rc() != 0 {
                false => "mtfsb0",
                true => "mtfsb0.",
            },
            Opcode::Mtfsb1 => match self.rc() != 0 {
                false => "mtfsb1",
                true => "mtfsb1.",
            },
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} crf{}", name, self.crf_d())
    }

    fn write_string_form_condreg12<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mcrf => "mcrf",
            Opcode::Mcrfs => "mcrfs",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} crf{}, crf{}", name, self.crf_d(), self.crf_s())
    }

    fn write_string_form_condreg123<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Crand => "crand",
            Opcode::Crandc => "crandc",
            Opcode::Creqv => "creqv",
            Opcode::Crnand => "crnand",
            Opcode::Crnor => "crnor",
            Opcode::Cror => "cror",
            Opcode::Crorc => "crorc",
            Opcode::Crxor => "crxor",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} crb{}, crb{}, crb{}",
            name,
            self.crb_d(),
            self.crb_a(),
            self.crb_b()
        )
    }

    fn write_string_form_reg23<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Dcbf => "dcbf",
            Opcode::Dcbi => "dcbi",
            Opcode::Dcbst => "dcbst",
            Opcode::Dcbt => "dcbt",
            Opcode::Dcbtst => "dcbtst",
            Opcode::Dcbz => "dcbz",
            Opcode::DcbzL => "dcbz_l",
            Opcode::Icbi => "icbi",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, r{}", name, self.a(), self.b())
    }

    fn write_string_form_reg213<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::Eqv => "eqv",
            Opcode::Nand => "nand",
            Opcode::Nor => "nor",
            Opcode::Or => {
                if self.s() == self.b() {
                    return write!(out, "mr r{}, r{}", self.a(), self.s());
                } else {
                    "or"
                }
            }
            Opcode::Orc => "orc",
            Opcode::Slw => "slw",
            Opcode::Sraw => "sraw",
            Opcode::Srw => "srw",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} r{}, r{}, r{}",
            name,
            name_suffix,
            self.a(),
            self.s(),
            self.b()
        )
    }

    fn write_string_rlw_imm<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name_prefix = if self.rc() != 0 { "." } else { "" };
        let name = match self.op {
            Opcode::Rlwimi => "rlwimi",
            Opcode::Rlwinm => "rlwinm",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{}{} r{}, r{}, {}, {}, {}",
            name,
            name_prefix,
            self.a(),
            self.s(),
            self.sh(),
            self.mb(),
            self.me()
        )
    }

    fn write_string_rlw_reg<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        assert_eq!(self.op, Opcode::Rlwnm);
        let name_prefix = if self.rc() != 0 { "." } else { "" };
        write!(
            out,
            "rlwnm{} r{}, r{}, r{}, {}, {}",
            name_prefix,
            self.a(),
            self.s(),
            self.b(),
            self.mb(),
            self.me()
        )
    }

    fn write_string_form_reg12_nb<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Lswi => "lswi",
            Opcode::Stswi => "stswi",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, r{}, {}", name, self.d(), self.a(), self.b())
    }

    fn write_string_form_reg1_spr<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mfspr => match self.spr() {
                1 => return write!(out, "mfxer r{}", self.s()),
                8 => return write!(out, "mflr r{}", self.s()),
                9 => return write!(out, "mfctr r{}", self.s()),
                _ => "mfspr",
            },
            Opcode::Mftb => "mftb",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, {}", name, self.d(), self.spr())
    }

    fn write_string_form_spr_reg1<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mtspr => match self.spr() {
                1 => return write!(out, "mtxer r{}", self.s()),
                8 => return write!(out, "mtlr r{}", self.s()),
                9 => return write!(out, "mtctr r{}", self.s()),
                _ => "mtspr",
            },
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} {}, r{}", name, self.spr(), self.s())
    }

    fn write_string_form_reg1_sr<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mfsr => "mfsr",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} r{}, {}", name, self.d(), self.sr())
    }

    fn write_string_form_sr_reg1<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::Mtsr => "mtsr",
            _ => disasm_unreachable!(self.code),
        };
        write!(out, "{} {}, r{}", name, self.sr(), self.s())
    }

    fn write_string_mtcrf<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        assert_eq!(self.op, Opcode::Mtcrf);
        write!(out, "mtcrf {} r{}", self.crm(), self.s())
    }

    fn write_string_srawi<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        assert_eq!(self.op, Opcode::Srawi);
        let name_suffix = if self.rc() != 0 { "." } else { "" };
        write!(
            out,
            "srawi{} r{}, r{}, {}",
            name_suffix,
            self.s(),
            self.a(),
            self.sh()
        )
    }

    fn write_string_tw<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        assert_eq!(self.op, Opcode::Tw);
        write!(out, "tw {}, r{}, r{}", self.to(), self.a(), self.b())
    }

    fn write_string_twi<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        assert_eq!(self.op, Opcode::Twi);
        write!(out, "twi {}, r{}, {}", self.to(), self.a(), self.simm())
    }

    fn write_string_psq<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::PsqL => "psq_l",
            Opcode::PsqLu => "psq_lu",
            Opcode::PsqSt => "psq_st",
            Opcode::PsqStu => "psq_stu",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} fr{}, {}(r{}), {}, qr{}",
            name,
            self.d(),
            self.ps_d(),
            self.a(),
            self.w(),
            self.ps_l()
        )
    }

    fn write_string_psq_x<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        let name = match self.op {
            Opcode::PsqLx => "psq_lx",
            Opcode::PsqLux => "psq_lux",
            Opcode::PsqStx => "psq_stx",
            Opcode::PsqStux => "psq_stux",
            _ => disasm_unreachable!(self.code),
        };
        write!(
            out,
            "{} fr{}, r{}, r{}, {}, {}",
            name,
            self.d(),
            self.a(),
            self.b(),
            self.w(),
            self.ps_l()
        )
    }

    pub fn write_string<W: Write>(&self, out: &mut W) -> std::io::Result<()> {
        match self.op {
            Opcode::Illegal => write!(out, "<illegal>"),

            // Standalone instructions
            Opcode::Eieio
            | Opcode::Isync
            | Opcode::Rfi
            | Opcode::Sc
            | Opcode::Sync
            | Opcode::Tlbsync => self.write_string_noargs(out),

            // General purpose register only
            Opcode::Mfcr | Opcode::Mfmsr | Opcode::Mtmsr | Opcode::Tlbie => {
                self.write_string_form_reg1(out)
            }
            Opcode::Addme | Opcode::Addze | Opcode::Neg | Opcode::Subfme | Opcode::Subfze => {
                self.write_string_form_reg12_oe_rc(out)
            }
            Opcode::Mfsrin | Opcode::Mtsrin => self.write_string_form_reg13(out),
            Opcode::Cntlzw | Opcode::Extsb | Opcode::Extsh => self.write_string_form_reg21_rc(out),
            Opcode::Dcbf
            | Opcode::Dcbi
            | Opcode::Dcbst
            | Opcode::Dcbt
            | Opcode::Dcbtst
            | Opcode::Dcbz
            | Opcode::DcbzL
            | Opcode::Icbi => self.write_string_form_reg23(out),
            Opcode::Eciwx
            | Opcode::Ecowx
            | Opcode::Lhaux
            | Opcode::Lhax
            | Opcode::Lbzux
            | Opcode::Lbzx
            | Opcode::Lhbrx
            | Opcode::Lhzux
            | Opcode::Lhzx
            | Opcode::Lswx
            | Opcode::Lwarx
            | Opcode::Lwbrx
            | Opcode::Lwzux
            | Opcode::Lwzx
            | Opcode::Stbx
            | Opcode::Stbux
            | Opcode::Sthbrx
            | Opcode::Sthx
            | Opcode::Sthux
            | Opcode::Stswx
            | Opcode::Stwbrx
            | Opcode::Stwcx_
            | Opcode::Stwx
            | Opcode::Stwux => self.write_string_form_reg123(out),
            Opcode::And | Opcode::Andc | Opcode::Mulhw | Opcode::Mulhwu | Opcode::Xor => {
                self.write_string_form_reg123_rc(out)
            }
            Opcode::Add
            | Opcode::Addc
            | Opcode::Adde
            | Opcode::Divw
            | Opcode::Divwu
            | Opcode::Mullw
            | Opcode::Subf
            | Opcode::Subfc
            | Opcode::Subfe => self.write_string_form_reg123_oe_rc(out),
            Opcode::Eqv
            | Opcode::Nand
            | Opcode::Nor
            | Opcode::Or
            | Opcode::Orc
            | Opcode::Slw
            | Opcode::Sraw
            | Opcode::Srw => self.write_string_form_reg213(out),

            // General purpose shifts
            Opcode::Rlwimi | Opcode::Rlwinm => self.write_string_rlw_imm(out),
            Opcode::Rlwnm => self.write_string_rlw_reg(out),

            // General purpose register misc
            Opcode::Addi
            | Opcode::Addic
            | Opcode::Addic_
            | Opcode::Addis
            | Opcode::Mulli
            | Opcode::Subfic => self.write_string_form_reg12_simm(out),
            Opcode::Andi_
            | Opcode::Andis_
            | Opcode::Ori
            | Opcode::Oris
            | Opcode::Xori
            | Opcode::Xoris => self.write_string_form_reg12_uimm(out),
            Opcode::Lbz
            | Opcode::Lbzu
            | Opcode::Lha
            | Opcode::Lhau
            | Opcode::Lhz
            | Opcode::Lhzu
            | Opcode::Lmw
            | Opcode::Lwz
            | Opcode::Lwzu
            | Opcode::Stb
            | Opcode::Stbu
            | Opcode::Sth
            | Opcode::Sthu
            | Opcode::Stmw
            | Opcode::Stw
            | Opcode::Stwu => self.write_string_form_reg12_offset(out),
            Opcode::Lswi | Opcode::Stswi => self.write_string_form_reg12_nb(out),
            Opcode::Mfspr | Opcode::Mftb => self.write_string_form_reg1_spr(out),
            Opcode::Mtspr => self.write_string_form_spr_reg1(out),
            Opcode::Mfsr => self.write_string_form_reg1_sr(out),
            Opcode::Mtsr => self.write_string_form_sr_reg1(out),
            Opcode::Mtcrf => self.write_string_mtcrf(out),
            Opcode::Srawi => self.write_string_srawi(out),
            Opcode::Tw => self.write_string_tw(out),
            Opcode::Twi => self.write_string_twi(out),

            // Branch instructions
            Opcode::B => self.write_string_b(out),
            Opcode::Bc => self.write_string_bc(out),
            Opcode::Bcctr | Opcode::Bclr => self.write_string_branch_cond_to_reg(out),

            // Compare instructions
            Opcode::Cmp | Opcode::Cmpl => self.write_string_cmp(out),
            Opcode::Cmpi => self.write_string_cmp_simm(out),
            Opcode::Cmpli => self.write_string_cmp_uimm(out),

            // Floating point register only instructions
            Opcode::Mffs => self.write_string_form_fr1(out),
            Opcode::Fabs
            | Opcode::Fmr
            | Opcode::Fnabs
            | Opcode::Fneg
            | Opcode::Fres
            | Opcode::Frsp
            | Opcode::Frsqrte
            | Opcode::PsAbs
            | Opcode::PsMr
            | Opcode::PsNabs
            | Opcode::PsNeg
            | Opcode::PsRes
            | Opcode::PsRsqrte => self.write_string_form_fr13(out),
            Opcode::Fadd
            | Opcode::Fadds
            | Opcode::Fdiv
            | Opcode::Fdivs
            | Opcode::Fsub
            | Opcode::Fsubs
            | Opcode::PsAdd
            | Opcode::PsDiv
            | Opcode::PsMerge00
            | Opcode::PsMerge01
            | Opcode::PsMerge10
            | Opcode::PsMerge11
            | Opcode::PsSub => self.write_string_form_fr123(out),
            Opcode::Fmul | Opcode::Fmuls | Opcode::PsMul | Opcode::PsMuls0 | Opcode::PsMuls1 => {
                self.write_string_form_fr124(out)
            }
            Opcode::Fmadd
            | Opcode::Fmadds
            | Opcode::Fmsub
            | Opcode::Fmsubs
            | Opcode::Fnmadd
            | Opcode::Fnmadds
            | Opcode::Fnmsub
            | Opcode::Fnmsubs
            | Opcode::Fsel
            | Opcode::PsMadd
            | Opcode::PsMadds0
            | Opcode::PsMadds1
            | Opcode::PsMsub
            | Opcode::PsNmadd
            | Opcode::PsNmsub
            | Opcode::PsSel
            | Opcode::PsSum0
            | Opcode::PsSum1 => self.write_string_form_fr1243(out),

            // Floating point register misc instructions
            Opcode::Fctiw | Opcode::Fctiwz => self.write_string_form_condreg1_fr13_rc(out),
            Opcode::Fcmpo
            | Opcode::Fcmpu
            | Opcode::PsCmpo0
            | Opcode::PsCmpo1
            | Opcode::PsCmpu0
            | Opcode::PsCmpu1 => self.write_string_form_condreg1_fr23(out),
            Opcode::Lfd
            | Opcode::Lfdu
            | Opcode::Lfs
            | Opcode::Lfsu
            | Opcode::Stfd
            | Opcode::Stfdu
            | Opcode::Stfs
            | Opcode::Stfsu => self.write_string_form_fr1_reg2_offset(out),
            Opcode::Lfdux
            | Opcode::Lfdx
            | Opcode::Lfsux
            | Opcode::Lfsx
            | Opcode::Stfdux
            | Opcode::Stfdx
            | Opcode::Stfiwx
            | Opcode::Stfsux
            | Opcode::Stfsx => self.write_string_form_fr1_reg23(out),
            Opcode::Mtfsf => self.write_string_mtfsf(out),

            // Condition register only
            Opcode::Mcrxr | Opcode::Mtfsb0 | Opcode::Mtfsb1 => self.write_string_form_condreg1(out),
            Opcode::Mcrf | Opcode::Mcrfs => self.write_string_form_condreg12(out),
            Opcode::Crand
            | Opcode::Crandc
            | Opcode::Creqv
            | Opcode::Crnand
            | Opcode::Crnor
            | Opcode::Cror
            | Opcode::Crorc
            | Opcode::Crxor => self.write_string_form_condreg123(out),

            // Condition register misc
            Opcode::Mtfsfi => self.write_string_mtfsfi(out),

            // Paired-single instructions
            Opcode::PsqL | Opcode::PsqLu | Opcode::PsqSt | Opcode::PsqStu => {
                self.write_string_psq(out)
            }
            Opcode::PsqLx | Opcode::PsqLux | Opcode::PsqStx | Opcode::PsqStux => {
                self.write_string_psq_x(out)
            }
        }
    }
}

impl ToString for Ins {
    fn to_string(&self) -> String {
        let mut buf = Vec::<u8>::new();
        self.write_string(&mut buf).unwrap();
        unsafe { String::from_utf8_unchecked(buf) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        assert_eq!(
            bits::<u32>(0b00000101100000000000000000000000u32, 5..9),
            0b1011u32
        );
        assert_eq!(bit(0b00000101100000000000000000000000u32, 5), 1);
    }
    #[test]
    fn test_opcodes() {
        macro_rules! assert_op {
            ($code:expr, $op:expr) => {{
                assert_eq!(Ins::disasm($code).op, $op)
            }};
        }

        // twi
        assert_op!(0b000011_00000_00000_0000000000000000, Opcode::Twi);
        // ps_cmpu0
        assert_op!(0b000100_00000_00000_00000_0000000000_0, Opcode::PsCmpu0);
        assert_op!(0b000100_00000_00000_00000_0000000000_1, Opcode::Illegal);
        assert_op!(0b000100_00001_00000_00000_0000000000_0, Opcode::Illegal);
        // psq_lx
        assert_op!(0b000100_00001_00000_00000_0000000110_0, Opcode::PsqLx);
        assert_op!(0b000100_00001_00000_00000_0000000110_1, Opcode::Illegal);
        assert_op!(0b000100_00001_00000_00000_0000000111_0, Opcode::PsqStx);
        assert_op!(0b000100_00001_00000_00000_0000000111_1, Opcode::Illegal);
        assert_op!(0x7c000278, Opcode::Xor);
        // TODO more tests
    }

    #[test]
    fn test_to_string() {
        macro_rules! assert_asm {
            ($code:expr, $disasm:expr) => {{
                assert_eq!(Ins::disasm($code).to_string(), $disasm)
            }};
        }
        assert_asm!(0x4c000000, "mcrf crf0, crf0");
        assert_asm!(0x7c000278, "xor r0, r0, r0");
        assert_asm!(0x10000014, "ps_sum0 fr0, fr0, fr0, fr0");
        assert_asm!(0x10000032, "ps_mul fr0, fr0, fr0");
        assert_asm!(0x7c00052a, "stswx r0, r0, r0");
        assert_asm!(0x9421ffc0, "stwu r1, -64(r1)");
        assert_asm!(0x7C0802A6, "mflr r0");
        assert_asm!(0x90010044, "stw r0, 68(r1)");
        assert_asm!(0xDBE10030, "stfd fr31, 48(r1)");
        assert_asm!(0xF3E10038, "psq_st fr31, 56(r1), 0, qr0");
        assert_asm!(0xDBC10020, "stfd fr30, 32(r1)");
        assert_asm!(0xF3C10028, "psq_st fr30, 40(r1), 0, qr0");
        assert_asm!(0xDBA10010, "stfd fr29, 16(r1)");
        assert_asm!(0xF3A10018, "psq_st fr29, 24(r1), 0, qr0");
        assert_asm!(0x93E1000C, "stw r31, 12(r1)");
        assert_asm!(0xFFE01890, "fmr fr31, fr3");
        assert_asm!(0x7C7F1B78, "mr r31, r3");
        assert_asm!(0xFFA00890, "fmr fr29, fr1");
        assert_asm!(0xFFC01090, "fmr fr30, fr2");
        assert_asm!(0xFC20F890, "fmr fr1, fr31");
        assert_asm!(0xEC3D0072, "fmuls fr1, fr29, fr1");
        assert_asm!(0xEC1D0772, "fmuls fr0, fr29, fr29");
        assert_asm!(0xEC5E0828, "fsubs fr2, fr30, fr1");
        assert_asm!(0xEC21007A, "fmadds fr1, fr1, fr1, fr0");
        assert_asm!(0xD05F0000, "stfs fr2, 0(r31)");
        assert_asm!(0xD03F0004, "stfs fr1, 4(r31)");
        assert_asm!(0xD3FF0008, "stfs fr31, 8(r31)");
        assert_asm!(0xE3E10038, "psq_l fr31, 56(r1), 0, qr0");
        assert_asm!(0xCBE10030, "lfd fr31, 48(r1)");
        assert_asm!(0xE3C10028, "psq_l fr30, 40(r1), 0, qr0");
        assert_asm!(0xCBC10020, "lfd fr30, 32(r1)");
        assert_asm!(0xE3A10018, "psq_l fr29, 24(r1), 0, qr0");
        assert_asm!(0xCBA10010, "lfd fr29, 16(r1)");
        assert_asm!(0x80010044, "lwz r0, 68(r1)");
        assert_asm!(0x83E1000C, "lwz r31, 12(r1)");
        assert_asm!(0x7C0803A6, "mtlr r0");
        assert_asm!(0x38210040, "addi r1, r1, 64");
        assert_asm!(0x4E800020, "blr");
    }
}
