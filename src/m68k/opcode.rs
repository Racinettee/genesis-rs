use std::fmt::{Display, Formatter, Result};

// Structure and data referenced from
// https://chromium.googlesource.com/native_client/nacl-gdb/+/refs/heads/main/opcodes/m68k-opc.c
#[derive(Debug)]
pub struct OpCode {
    name: &'static str,
    args: &'static str,
    opcode: u32,
    mask: u32,
    size: u16,
    arch: u16,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}, {:#04X}, mask: {:#04X} size: {} arch: {}", self.name, self.args, self.opcode, self.mask, self.size, self.arch)
    }
}

pub const M68KUP: u16 = 1;

const fn one(x: u32) -> u32 {
    x << 16
}

const fn two(x: u32, y: u32) -> u32 {
    (x << 16) + y
}

pub static OP_CODES: &[OpCode] = &[
    OpCode{name: "abcd", args: "DsDd", opcode: one(0140400), mask: one(0170770), size: 2, arch: M68KUP},
    OpCode{name: "abcd", args: "-s-d", opcode: one(0140410), mask: one(0170770), size: 2, arch: M68KUP},
];
