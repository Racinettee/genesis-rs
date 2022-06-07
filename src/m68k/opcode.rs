use std::fmt::{Display, Formatter, Result};

use super::arch;

// Structure and data referenced from
// https://chromium.googlesource.com/native_client/nacl-gdb/+/refs/heads/main/opcodes/m68k-opc.c
#[derive(Debug)]
pub struct OpCode {
    name: &'static str,
    args: &'static str,
    opcode: u32,
    mask: u32,
    arch: u32,
    size: u16,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}, {:#04X}, mask: {:#04X} size: {} arch: {}", self.name, self.args, self.opcode, self.mask, self.size, self.arch)
    }
}

const fn one(x: u32) -> u32 {
    x << 16
}

const fn two(x: u32, y: u32) -> u32 {
    (x << 16) + y
}

const SCOPE_LINE: u32 = 0x1 << 3;
const SCOPE_PAGE: u32 = 0x2 << 3;
const SCOPE_ALL: u32 = 0x3 << 3;

pub static OP_CODES: &[OpCode] = &[
    OpCode{name: "abcd", size: 2, opcode: one(0140400), mask: one(0170770), args: "DsDd", arch: arch::M68000UP},
    OpCode{name: "abcd", size: 2, opcode: one(0140410), mask: one(0170770), args: "-s-d", arch: arch::M68000UP},

    OpCode{name: "addaw", size: 2, opcode: one(0150300), mask: one(0170700), args: "*wAd", arch: arch::M68000UP},
    OpCode{name: "addal", size: 2, opcode: one(0150700), mask: one(0170700), args: "*lAd", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "addib", size: 4, opcode: one(0003000), mask: one(0177700), args: "#b$s", arch: arch::M68000UP},
    OpCode{name: "addiw", size: 4, opcode: one(0003100), mask: one(0177700), args: "#w$s", arch: arch::M68000UP},
    OpCode{name: "addil", size: 6, opcode: one(0003200), mask: one(0177700), args: "#l$s", arch: arch::M68000UP},
    OpCode{name: "addil", size: 6, opcode: one(0003200), mask: one(0177700), args: "#lDs", arch: arch::MCFISA_A},

    OpCode{name: "addqb", size: 2, opcode: one(0050000), mask: one(0170700), args: "Qd$b", arch: arch::M68000UP},
    OpCode{name: "addqw", size: 2, opcode: one(0050100), mask: one(0170700), args: "Qd%w", arch: arch::M68000UP},
    OpCode{name: "addql", size: 2, opcode: one(0050200), mask: one(0170700), args: "Qd%l", arch: arch::M68000UP | arch::MCFISA_A},

    // The add opcode can generate the adda, addi, and addq instructions.
    OpCode{name: "addb", size: 2, opcode: one(0050000), mask: one(0170700), args: "Qd$b", arch: arch::M68000UP},
    OpCode{name: "addb", size: 4, opcode: one(0003000), mask: one(0177700), args: "#b$s", arch: arch::M68000UP},
    OpCode{name: "addb", size: 2, opcode: one(0150000), mask: one(0170700), args: ";bDd", arch: arch::M68000UP},
    OpCode{name: "addb", size: 2, opcode: one(0150400), mask: one(0170700), args: "Dd~b", arch: arch::M68000UP},
    OpCode{name: "addw", size: 2, opcode: one(0050100), mask: one(0170700), args: "Qd%w", arch: arch::M68000UP},
    OpCode{name: "addw", size: 2, opcode: one(0150300), mask: one(0170700), args: "*wAd", arch: arch::M68000UP},
    OpCode{name: "addw", size: 4, opcode: one(0003100), mask: one(0177700), args: "#w$s", arch: arch::M68000UP},
    OpCode{name: "addw", size: 2, opcode: one(0150100), mask: one(0170700), args: "*wDd", arch: arch::M68000UP},
    OpCode{name: "addw", size: 2, opcode: one(0150500), mask: one(0170700), args: "Dd~w", arch: arch::M68000UP},
    OpCode{name: "addl", size: 2, opcode: one(0050200), mask: one(0170700), args: "Qd%l", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "addl", size: 6, opcode: one(0003200), mask: one(0177700), args: "#l$s", arch: arch::M68000UP},
    OpCode{name: "addl", size: 6, opcode: one(0003200), mask: one(0177700), args: "#lDs", arch: arch::MCFISA_A},
    OpCode{name: "addl", size: 2, opcode: one(0150700), mask: one(0170700), args: "*lAd", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "addl", size: 2, opcode: one(0150200), mask: one(0170700), args: "*lDd", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "addl", size: 2, opcode: one(0150600), mask: one(0170700), args: "Dd~l", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "addxb", size: 2, opcode: one(0150400), mask: one(0170770), args: "DsDd", arch: arch::M68000UP},
    OpCode{name: "addxb", size: 2, opcode: one(0150410), mask: one(0170770), args: "-s-d", arch: arch::M68000UP},
    OpCode{name: "addxw", size: 2, opcode: one(0150500), mask: one(0170770), args: "DsDd", arch: arch::M68000UP},
    OpCode{name: "addxw", size: 2, opcode: one(0150510), mask: one(0170770), args: "-s-d", arch: arch::M68000UP},
    OpCode{name: "addxl", size: 2, opcode: one(0150600), mask: one(0170770), args: "DsDd", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "addxl", size: 2, opcode: one(0150610), mask: one(0170770), args: "-s-d", arch: arch::M68000UP},

    OpCode{name: "andib", size: 4, opcode: one(0001000), mask: one(0177700), args: "#b$s", arch: arch::M68000UP},
    OpCode{name: "andib", size: 4, opcode: one(0001074), mask: one(0177777), args: "#bCs", arch: arch::M68000UP},
    OpCode{name: "andiw", size: 4, opcode: one(0001100), mask: one(0177700), args: "#w$s", arch: arch::M68000UP},
    OpCode{name: "andiw", size: 4, opcode: one(0001174), mask: one(0177777), args: "#wSs", arch: arch::M68000UP},
    OpCode{name: "andil", size: 6, opcode: one(0001200), mask: one(0177700), args: "#l$s", arch: arch::M68000UP},
    OpCode{name: "andil", size: 6, opcode: one(0001200), mask: one(0177700), args: "#lDs", arch: arch::MCFISA_A},
    OpCode{name: "andi", size: 4, opcode: one(0001100), mask: one(0177700), args: "#w$s", arch: arch::M68000UP},
    OpCode{name: "andi", size: 4, opcode: one(0001074), mask: one(0177777), args: "#bCs", arch: arch::M68000UP},
    OpCode{name: "andi", size: 4, opcode: one(0001174), mask: one(0177777), args: "#wSs", arch: arch::M68000UP},

    // The and opcode can generate the andi instruction.
    OpCode{name: "andb", size: 4, opcode: one(0001000),	mask: one(0177700), args: "#b$s", arch: arch::M68000UP},
    OpCode{name: "andb", size: 4, opcode: one(0001074),	mask: one(0177777), args: "#bCs", arch: arch::M68000UP},
    OpCode{name: "andb", size: 2, opcode: one(0140000),	mask: one(0170700), args: ";bDd", arch: arch::M68000UP},
    OpCode{name: "andb", size: 2, opcode: one(0140400),	mask: one(0170700), args: "Dd~b", arch: arch::M68000UP},
    OpCode{name: "andw", size: 4, opcode: one(0001100),	mask: one(0177700), args: "#w$s", arch: arch::M68000UP},
    OpCode{name: "andw", size: 4, opcode: one(0001174),	mask: one(0177777), args: "#wSs", arch: arch::M68000UP},
    OpCode{name: "andw", size: 2, opcode: one(0140100),	mask: one(0170700), args: ";wDd", arch: arch::M68000UP},
    OpCode{name: "andw", size: 2, opcode: one(0140500),	mask: one(0170700), args: "Dd~w", arch: arch::M68000UP},
    OpCode{name: "andl", size: 6, opcode: one(0001200),	mask: one(0177700), args: "#l$s", arch: arch::M68000UP},
    OpCode{name: "andl", size: 6, opcode: one(0001200),	mask: one(0177700), args: "#lDs", arch: arch::MCFISA_A},
    OpCode{name: "andl", size: 2, opcode: one(0140200),	mask: one(0170700), args: ";lDd", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "andl", size: 2, opcode: one(0140600),	mask: one(0170700), args: "Dd~l", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "and", size: 4, opcode: one(0001100),	mask: one(0177700), args: "#w$w", arch: arch::M68000UP},
    OpCode{name: "and", size: 4, opcode: one(0001074),	mask: one(0177777), args: "#bCs", arch: arch::M68000UP},
    OpCode{name: "and", size: 4, opcode: one(0001174),	mask: one(0177777), args: "#wSs", arch: arch::M68000UP},
    OpCode{name: "and", size: 2, opcode: one(0140100),	mask: one(0170700), args: ";wDd", arch: arch::M68000UP},
    OpCode{name: "and", size: 2, opcode: one(0140500),	mask: one(0170700), args: "Dd~w", arch: arch::M68000UP},

    OpCode{name: "aslb", size: 2, opcode: one(0160400),	mask: one(0170770), args: "QdDs", arch: arch::M68000UP},
    OpCode{name: "aslb", size: 2, opcode: one(0160440),	mask: one(0170770), args: "DdDs", arch: arch::M68000UP},
    OpCode{name: "aslw", size: 2, opcode: one(0160500),	mask: one(0170770), args: "QdDs", arch: arch::M68000UP},
    OpCode{name: "aslw", size: 2, opcode: one(0160540),	mask: one(0170770), args: "DdDs", arch: arch::M68000UP},
    OpCode{name: "aslw", size: 2, opcode: one(0160700),	mask: one(0177700), args: "~s",   arch: arch::M68000UP },
    OpCode{name: "asll", size: 2, opcode: one(0160600),	mask: one(0170770), args: "QdDs", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "asll", size: 2, opcode: one(0160640),	mask: one(0170770), args: "DdDs", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "asrb", size: 2, opcode: one(0160000),	mask: one(0170770), args: "QdDs", arch: arch::M68000UP},
    OpCode{name: "asrb", size: 2, opcode: one(0160040),	mask: one(0170770), args: "DdDs", arch: arch::M68000UP},
    OpCode{name: "asrw", size: 2, opcode: one(0160100),	mask: one(0170770), args: "QdDs", arch: arch::M68000UP},
    OpCode{name: "asrw", size: 2, opcode: one(0160140),	mask: one(0170770), args: "DdDs", arch: arch::M68000UP},
    OpCode{name: "asrw", size: 2, opcode: one(0160300),	mask: one(0177700), args: "~s",   arch: arch::M68000UP},
    OpCode{name: "asrl", size: 2, opcode: one(0160200),	mask: one(0170770), args: "QdDs", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "asrl", size: 2, opcode: one(0160240),	mask: one(0170770), args: "DdDs", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "bhiw", size: 2, opcode: one(0061000),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "blsw", size: 2, opcode: one(0061400),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bccw", size: 2, opcode: one(0062000),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bcsw", size: 2, opcode: one(0062400),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bnew", size: 2, opcode: one(0063000),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "beqw", size: 2, opcode: one(0063400),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bvcw", size: 2, opcode: one(0064000),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bvsw", size: 2, opcode: one(0064400),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bplw", size: 2, opcode: one(0065000),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bmiw", size: 2, opcode: one(0065400),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bgew", size: 2, opcode: one(0066000),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bltw", size: 2, opcode: one(0066400),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bgtw", size: 2, opcode: one(0067000),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "blew", size: 2, opcode: one(0067400),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "bhil", size: 2, opcode: one(0061377),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "blsl", size: 2, opcode: one(0061777),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bccl", size: 2, opcode: one(0062377),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bcsl", size: 2, opcode: one(0062777),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bnel", size: 2, opcode: one(0063377),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "beql", size: 2, opcode: one(0063777),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bvcl", size: 2, opcode: one(0064377),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bvsl", size: 2, opcode: one(0064777),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bpll", size: 2, opcode: one(0065377),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bmil", size: 2, opcode: one(0065777),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bgel", size: 2, opcode: one(0066377),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bltl", size: 2, opcode: one(0066777),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bgtl", size: 2, opcode: one(0067377),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "blel", size: 2, opcode: one(0067777),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},

    OpCode{name: "bhis", size: 2, opcode: one(0061000),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "blss", size: 2, opcode: one(0061400),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bccs", size: 2, opcode: one(0062000),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bcss", size: 2, opcode: one(0062400),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bnes", size: 2, opcode: one(0063000),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "beqs", size: 2, opcode: one(0063400),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bvcs", size: 2, opcode: one(0064000),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bvss", size: 2, opcode: one(0064400),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bpls", size: 2, opcode: one(0065000),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bmis", size: 2, opcode: one(0065400),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bges", size: 2, opcode: one(0066000),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "blts", size: 2, opcode: one(0066400),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bgts", size: 2, opcode: one(0067000),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bles", size: 2, opcode: one(0067400),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "jhi", size: 2, opcode: one(0061000),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jls", size: 2, opcode: one(0061400),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jcc", size: 2, opcode: one(0062000),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jcs", size: 2, opcode: one(0062400),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jne", size: 2, opcode: one(0063000),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jeq", size: 2, opcode: one(0063400),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jvc", size: 2, opcode: one(0064000),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jvs", size: 2, opcode: one(0064400),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jpl", size: 2, opcode: one(0065000),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jmi", size: 2, opcode: one(0065400),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jge", size: 2, opcode: one(0066000),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jlt", size: 2, opcode: one(0066400),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jgt", size: 2, opcode: one(0067000),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "jle", size: 2, opcode: one(0067400),	mask: one(0177400), args: "Bg", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "bchg", size: 2, opcode: one(0000500),	mask: one(0170700), args: "Dd$s", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bchg", size: 4, opcode: one(0004100),	mask: one(0177700), args: "#b$s", arch: arch::M68000UP},
    OpCode{name: "bchg", size: 4, opcode: one(0004100),	mask: one(0177700), args: "#bqs", arch: arch::MCFISA_A},

    OpCode{name: "bclr", size: 2, opcode: one(0000600),	mask: one(0170700), args: "Dd$s", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bclr", size: 4, opcode: one(0004200),	mask: one(0177700), args: "#b$s", arch: arch::M68000UP},
    OpCode{name: "bclr", size: 4, opcode: one(0004200),	mask: one(0177700), args: "#bqs", arch: arch::MCFISA_A},

    OpCode{name: "bfchg", size: 4, opcode: two(0165300, 0), mask: two(0177700, 0170000), args: "?sO2O3",  arch: arch::M68020UP},
    OpCode{name: "bfclr", size: 4, opcode: two(0166300, 0), mask: two(0177700, 0170000), args: "?sO2O3",   arch: arch::M68020UP},
    OpCode{name: "bfexts", size: 4, opcode: two(0165700, 0), mask: two(0177700, 0100000), args: "/sO2O3D1", arch: arch::M68020UP},
    OpCode{name: "bfextu", size: 4, opcode: two(0164700, 0), mask: two(0177700, 0100000), args: "/sO2O3D1", arch: arch::M68020UP},
    OpCode{name: "bfffo", size: 4, opcode: two(0166700, 0), mask: two(0177700, 0100000), args: "/sO2O3D1", arch: arch::M68020UP},
    OpCode{name: "bfins", size: 4, opcode: two(0167700, 0), mask: two(0177700, 0100000), args: "D1?sO2O3", arch: arch::M68020UP},
    OpCode{name: "bfset", size: 4, opcode: two(0167300, 0), mask: two(0177700, 0170000), args: "?sO2O3",   arch: arch::M68020UP},
    OpCode{name: "bftst", size: 4, opcode: two(0164300, 0), mask: two(0177700, 0170000), args: "/sO2O3",   arch: arch::M68020UP},

    OpCode{name: "bgnd", size: 2, opcode: one(0045372),	mask: one(0177777), args: "", arch: arch::CPU32 | arch::FIDO_A},

    OpCode{name: "bitrev", size: 2, opcode: one(0000300),	mask: one(0177770), args: "Ds", arch: arch::MCFISA_AA | arch::MCFISA_C},

    OpCode{name: "bkpt", size: 2, opcode: one(0044110),	mask: one(0177770), args: "ts", arch: arch::M68010UP},

    OpCode{name: "braw", size: 2, opcode: one(0060000),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bral", size: 2, opcode: one(0060377),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B},
    OpCode{name: "bras", size: 2, opcode: one(0060000),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "bset", size: 2, opcode: one(0000700),	mask: one(0170700), args: "Dd$s", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bset", size: 2, opcode: one(0000700),	mask: one(0170700), args: "Ddvs", arch: arch::MCFISA_A},
    OpCode{name: "bset", size: 4, opcode: one(0004300),	mask: one(0177700), args: "#b$s", arch: arch::M68000UP},
    OpCode{name: "bset", size: 4, opcode: one(0004300),	mask: one(0177700), args: "#bqs", arch: arch::MCFISA_A},

    OpCode{name: "bsrw", size: 2, opcode: one(0060400),	mask: one(0177777), args: "BW", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "bsrl", size: 2, opcode: one(0060777),	mask: one(0177777), args: "BL", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A | arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "bsrs", size: 2, opcode: one(0060400),	mask: one(0177400), args: "BB", arch: arch::M68000UP | arch::MCFISA_A},

    OpCode{name: "btst", size: 2, opcode: one(0000400),	mask: one(0170700), args: "Dd;b", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "btst", size: 4, opcode: one(0004000),	mask: one(0177700), args: "#b@s", arch: arch::M68000UP},
    OpCode{name: "btst", size: 4, opcode: one(0004000),	mask: one(0177700), args: "#bqs", arch: arch::MCFISA_A},

    OpCode{name: "byterev", size: 2, opcode: one(0001300), mask: one(0177770), args: "Ds", arch: arch::MCFISA_A | arch::MCFISA_C},

    OpCode{name: "callm", size: 4, opcode: one(0003300),   mask: one(0177700), args: "#b!s", arch: arch::M68020},

    OpCode{name: "cas2w", size: 6, opcode: two(0006374,0), mask: two(0177777,0007070), args: "D3D6D2D5R1R4", arch: arch::M68020UP},
    OpCode{name: "cas2w", size: 6, opcode: two(0006374,0), mask: two(0177777,0007070), args: "D3D6D2D5r1r4", arch: arch::M68020UP},
    OpCode{name: "cas2l", size: 6, opcode: two(0007374,0), mask: two(0177777,0007070), args: "D3D6D2D5r1r4", arch: arch::M68020UP},
    OpCode{name: "cas2l", size: 6, opcode: two(0007374,0), mask: two(0177777,0007070), args: "D3D6D2D5R1R4", arch: arch::M68020UP},

    OpCode{name: "casb", size: 4, opcode: two(0005300, 0), mask: two(0177700, 0177070), args: "D3D2~s", arch: arch::M68020UP},
    OpCode{name: "casw", size: 4, opcode: two(0006300, 0), mask: two(0177700, 0177070),	args: "D3D2~s", arch: arch::M68020UP},
    OpCode{name: "casl", size: 4, opcode: two(0007300, 0), mask: two(0177700, 0177070),	args: "D3D2~s", arch: arch::M68020UP},

    OpCode{name: "chk2b", size: 4, opcode: 	two(0000300,0004000), mask: two(0177700,07777), args: "!sR1", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A},
    OpCode{name: "chk2w", size: 4, opcode: 	two(0001300,0004000),	mask: two(0177700,07777), args: "!sR1", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A},
    OpCode{name: "chk2l", size: 4, opcode: 	two(0002300,0004000),	mask: two(0177700,07777), args: "!sR1", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A},

    OpCode{name: "chkl", size: 2, opcode: one(0040400),	mask: one(0170700), args: ";lDd", arch: arch::M68000UP},
    OpCode{name: "chkw", size: 2, opcode: one(0040600),	mask: one(0170700), args: ";wDd", arch: arch::M68000UP},

    OpCode{name: "cinva", size: 2, opcode: one(0xf400|SCOPE_ALL), mask: one(0xff38), args: "ce", arch: arch::M68040UP},
    OpCode{name: "cinvl", size: 2, opcode: one(0xf400|SCOPE_LINE), mask: one(0xff38), args: "ceas", arch: arch::M68040UP},
    OpCode{name: "cinvp", size: 2, opcode: one(0xf400|SCOPE_PAGE), mask: one(0xff38), args: "ceas", arch: arch::M68040UP},
    OpCode{name: "cpusha", size: 2, opcode: one(0xf420|SCOPE_ALL), mask: one(0xff38), args: "ce", arch: arch::M68040UP},
    OpCode{name: "cpushl", size: 2, opcode: one(0xf420|SCOPE_LINE), mask: one(0xff38), args: "ceas", arch: arch::M68040UP | arch::MCFISA_A},
    OpCode{name: "cpushp", size: 2, opcode: one(0xf420|SCOPE_PAGE), mask: one(0xff38), args: "ceas", arch: arch::M68040UP},

    OpCode{name: "clrb", size: 2, opcode: one(0041000), mask: one(0177700), args: "$s", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "clrw", size: 2, opcode: one(0041100), mask: one(0177700), args: "$s", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "clrl", size: 2, opcode: one(0041200), mask: one(0177700), args: "$s", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "cmp2b", size: 4, opcode: two(0000300,0), mask: two(0177700,07777), args: "!sR1", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A},
    OpCode{name: "cmp2w", size: 4, opcode: two(0001300,0), mask: two(0177700,07777), args: "!sR1", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A},
    OpCode{name: "cmp2l", size: 4, opcode: two(0002300,0), mask: two(0177700,07777), args: "!sR1", arch: arch::M68020UP | arch::CPU32 | arch::FIDO_A},
    OpCode{name: "cmpaw", size: 2, opcode: one(0130300), mask: one(0170700), args: "*wAd", arch: arch::M68000UP},
    OpCode{name: "cmpal", size: 2, opcode: one(0130700), mask: one(0170700), args: "*lAd", arch: arch::M68000UP | arch::MCFISA_A},
    OpCode{name: "cmpib", size: 4, opcode: one(0006000), mask: one(0177700), args: "#b@s", arch: arch::M68000UP},
    OpCode{name: "cmpib", size: 4, opcode: one(0006000), mask: one(0177700), args: "#bDs", arch: arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "cmpiw", size: 4, opcode: one(0006100), mask: one(0177700), args: "#w@s", arch: arch::M68000UP},
    OpCode{name: "cmpiw", size: 4, opcode: one(0006100), mask: one(0177700), args: "#wDs", arch: arch::MCFISA_B | arch::MCFISA_C},
    OpCode{name: "cmpil", size: 6, opcode: one(0006200), mask: one(0177700), args: "#l@s", arch: arch::M68000UP},
    OpCode{name: "cmpil", size: 6, opcode: one(0006200), mask: one(0177700), args: "#lDs", arch: arch::MCFISA_A},
    OpCode{name: "cmpmb", size: 2, opcode: one(0130410), mask: one(0170770), args: "+s+d", arch: arch::M68000UP},
    OpCode{name: "cmpmw", size: 2, opcode: one(0130510), mask: one(0170770), args: "+s+d", arch: arch::M68000UP},
    OpCode{name: "cmpml", size: 2, opcode: one(0130610), mask: one(0170770), args: "+s+d", arch: arch::M68000UP},
/* The cmp opcode can generate the cmpa, cmpm, and cmpi instructions.  */
];
