use std::fmt;

/// The interface for representing an A64 instruction.
pub trait Instruction {
    /// The instruction's expected configuration of bits.
    const REQUIRED_BITS: u32;

    /// The bitmask for testing the instruction's expected configuration of bits.
    const REQUIRED_BITS_MASK: u32;

    /// Determines if the machine code fits the instruction.
    fn test(instr: u32) -> bool {
        instr & Self::REQUIRED_BITS_MASK == Self::REQUIRED_BITS
    }

    /// Extracts the operands from the given bytecode, returning `None` if the bytecode does not fit the instruction.
    fn decode(instr: u32) -> Option<Self>
    where
        Self: Sized;
}

/// The instruction for adding a register value and an immediate value.
pub struct AddImm {
    /// The general-purpose destination register.
    pub rd: u8,

    /// The general-purpose source register.
    pub rn: u8,

    /// The 12-bit unsigned immediate.
    pub imm12: u16,

    /// Determines if the immediate should be shifted to the left by 12 bits.
    pub sh: bool,

    /// Determines if the instruction should operate on data in 64 bits.
    pub sf: bool,
}

impl Instruction for AddImm {
    #[allow(clippy::unusual_byte_groupings)]
    const REQUIRED_BITS: u32 = 0b0_0_0_100010_0_000000000000_00000_00000;

    #[allow(clippy::unusual_byte_groupings)]
    const REQUIRED_BITS_MASK: u32 = 0b0_1_1_111111_0_000000000000_00000_00000;

    fn decode(instr: u32) -> Option<Self> {
        if !Self::test(instr) {
            return None;
        }

        let rd = (instr & 0x1F) as u8;
        let rn = ((instr >> 5) & 0x1F) as u8;
        let imm12 = ((instr >> 10) & 0xFFF) as u16;
        let sh = ((instr >> 22) & 0x1) != 0;
        let sf = (instr >> 31) != 0;

        Some(Self {
            rd,
            rn,
            imm12,
            sh,
            sf,
        })
    }
}

impl fmt::Display for AddImm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.sf {
            write!(
                f,
                "add x{}, x{}, #{:#x?}, LSL #{}",
                self.rd,
                self.rn,
                self.imm12,
                if self.sh { 12 } else { 0 },
            )
        } else {
            write!(
                f,
                "add w{}, w{}, #{:#x?}, LSL #{}",
                self.rd,
                self.rn,
                self.imm12,
                if self.sh { 12 } else { 0 },
            )
        }
    }
}

/// The instruction for computing the 4 KB page address of a program label.
pub struct Adrp {
    /// The general-purpose destination register.
    pub rd: u8,

    /// The 21-bit signed immediate.
    pub imm: u32,
}

impl Instruction for Adrp {
    #[allow(clippy::unusual_byte_groupings)]
    const REQUIRED_BITS: u32 = 0b1_00_10000_0000000000000000000_00000;

    #[allow(clippy::unusual_byte_groupings)]
    const REQUIRED_BITS_MASK: u32 = 0b1_00_11111_0000000000000000000_00000;

    fn decode(instr: u32) -> Option<Self> {
        if !Self::test(instr) {
            return None;
        }

        let rd = (instr & 0x1F) as u8;
        let immhi = (instr >> 5) & 0x7FFFF;
        let immlo = (instr >> 29) & 0x3;
        let imm = (immhi << 14) + (immlo << 12);

        Some(Self { rd, imm })
    }
}

impl fmt::Display for Adrp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "adrp x{}, #{:#x?}", self.rd, self.imm)
    }
}
