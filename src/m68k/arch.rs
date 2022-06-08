pub const M68000: u32 =   0x001;
pub const M68010: u32 =   0x002;
pub const M68020: u32 =   0x004;
pub const M68030: u32 =   0x008;
pub const M68040: u32 =   0x010;
pub const M68060: u32 =   0x020;
pub const M68881: u32 =   0x040;
pub const M68851: u32 =   0x080;
pub const CPU32: u32 =	 0x100		/* e.g., 68332 */;
pub const FIDO_A: u32 =   0x200;
pub const M68K_MASK: u32 =  0x3ff;
pub const MCFMAC: u32 =   0x400		/* ColdFire MAC. */;
pub const MCFEMAC: u32 =  0x800		/* ColdFire EMAC. */;
pub const CFLOAT: u32 =   0x1000		/* ColdFire FPU.  */;
pub const MCFHWDIV: u32 = 0x2000		/* ColdFire hardware divide.  */;
pub const MCFISA_A: u32 = 0x4000		/* ColdFire ISA_A.  */;
pub const MCFISA_AA: u32 = 0x8000	/* ColdFire ISA_A+.  */;
pub const MCFISA_B: u32 = 0x10000	/* ColdFire ISA_B.  */;
pub const MCFISA_C: u32 = 0x20000	/* ColdFire ISA_C.  */;
pub const MCFUSP: u32 =   0x40000	/* ColdFire USP instructions.  */;
pub const MCF_MASK: u32 = 0x7e400;
/* Handy aliases.  */
pub const M68040UP: u32 =   M68040 | M68060;
pub const M68030UP: u32 =   M68030 | M68040UP;
pub const M68020UP: u32 =   M68020 | M68030UP;
pub const M68010UP: u32 =   M68010 | CPU32 | FIDO_A | M68020UP;
pub const M68000UP: u32 =   M68000 | M68010UP;
pub const MFLOAT: u32 =  M68881 | M68040 | M68060;
pub const MMMU: u32 =    M68851 | M68030 | M68040 | M68060;