
use crate::isa::*;

/// RV32I instruction formats.
#[derive(Debug)]
pub enum InstFormat { R, I, S, B, U, J }

/// RV32I opcodes.
#[repr(usize)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    LOAD       = 0b00000, // [lb, lh, lw, lbu, lhu]
    LOAD_FP    = 0b00001,
    CUSTOM_0   = 0b00010,
    MISC_MEM   = 0b00011, // [fence, fence.i]
    OP_IMM     = 0b00100, // [addi, slti, sltiu, xori, ori, andi]
    AUIPC      = 0b00101, 
    OP_IMM_32  = 0b00110,
    STORE      = 0b01000, // [sb, sh, sw]
    STORE_FP   = 0b01001,
    CUSTOM_1   = 0b01010,
    AMO        = 0b01011,
    OP         = 0b01100, // [add, sub, sll, slt, sltu, xor, srl, sra, or, and]
    LUI        = 0b01101,
    OP_32      = 0b01110,
    MADD       = 0b10000,
    MSUB       = 0b10001,
    NMSUB      = 0b10010,
    NMADD      = 0b10011,
    OP_FP      = 0b10100,
    RES_0      = 0b10101,
    CUSTOM_2   = 0b10110,
    BRANCH     = 0b11000, // [beq, bne, blt, bge, bltu, bgeu]
    JALR       = 0b11001,
    RES_1      = 0b11010,
    JAL        = 0b11011,
    SYSTEM     = 0b11100,
    RES_2      = 0b11101,
    CUSTOM_3   = 0b11110,
}
impl From<u32> for Opcode {
    fn from(x: u32) -> Self {
        match x {
         0b00000 => Self::LOAD,
         0b00001 => Self::LOAD_FP,
         0b00010 => Self::CUSTOM_0,
         0b00011 => Self::MISC_MEM,
         0b00100 => Self::OP_IMM,
         0b00101 => Self::AUIPC,
         0b00110 => Self::OP_IMM_32,
         0b01000 => Self::STORE,
         0b01001 => Self::STORE_FP,
         0b01010 => Self::CUSTOM_1,
         0b01011 => Self::AMO,
         0b01100 => Self::OP,
         0b01101 => Self::LUI,
         0b01110 => Self::OP_32,
         0b10000 => Self::MADD,
         0b10001 => Self::MSUB,
         0b10010 => Self::NMSUB,
         0b10011 => Self::NMADD,
         0b10100 => Self::OP_FP,
         0b10101 => Self::RES_0,
         0b10110 => Self::CUSTOM_2,
         0b11000 => Self::BRANCH,
         0b11001 => Self::JALR,
         0b11010 => Self::RES_1,
         0b11011 => Self::JAL,
         0b11100 => Self::SYSTEM,
         0b11101 => Self::RES_2,
         0b11110 => Self::CUSTOM_3,
         _ => unimplemented!(),
        }
    }
}

/// ALU opcodes for I-type encodings.
#[derive(Debug)]
pub enum RvALUOpImm { Addi, Slti, Sltiu, Xori, Ori, Andi, Slli, Srli, Srai }
impl From<(u32, u32)> for RvALUOpImm {
    fn from(x: (u32, u32)) -> Self {
        match x {
            (0b000, _) => Self::Addi,
            (0b010, _) => Self::Slti,
            (0b011, _) => Self::Sltiu,
            (0b100, _) => Self::Xori,
            (0b110, _) => Self::Ori,
            (0b111, _) => Self::Andi,

            (0b001, 0b0000000) => Self::Slli,
            (0b101, 0b0000000) => Self::Srli,
            (0b101, 0b0100000) => Self::Srai,
            _ => unimplemented!("ALU op f3={:03b} f7={:07b}", x.0, x.1),
        }
    }
}
impl std::fmt::Display for RvALUOpImm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Addi  => "addi",
            Self::Slti  => "slti",
            Self::Sltiu => "sltiu",
            Self::Xori => "xori",
            Self::Ori => "ori",
            Self::Andi => "andi",
            Self::Slli => "slli",
            Self::Srli => "srli",
            Self::Srai => "srai",
        };
        write!(f, "{}", s)
    }
}



/// ALU opcodes for R-type encodings.
#[derive(Debug)]
pub enum RvALUOp { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }
impl From<(u32, u32)> for RvALUOp {
    fn from(x: (u32, u32)) -> Self {
        match x {
            (0b000, 0b0000000) => Self::Add,
            (0b000, 0b0100000) => Self::Sub,

            (0b001, 0b0000000) => Self::Sll,
            (0b010, 0b0000000) => Self::Slt,
            (0b011, 0b0000000) => Self::Sltu,
            (0b100, 0b0000000) => Self::Xor,

            (0b101, 0b0000000) => Self::Srl,
            (0b101, 0b0100000) => Self::Sra,

            (0b110, 0b0000000) => Self::Or,
            (0b111, 0b0000000) => Self::And,
            _ => unimplemented!("ALU op f3={:03b} f7[1]={}", x.0, x.1),
        }
    }
}
impl std::fmt::Display for RvALUOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Add  => "add",
            Self::Sub  => "sub",
            Self::Sll  => "sll",
            Self::Slt  => "slt",
            Self::Sltu => "sltu",
            Self::Xor  => "xor",
            Self::Srl  => "srl",
            Self::Sra  => "sra",
            Self::Or   => "or",
            Self::And  => "and",
        };
        write!(f, "{}", s)
    }
}


/// RV32I load/store width encodings.
#[derive(Debug)]
pub enum RvWidth { Byte, Half, Word, ByteUnsigned, HalfUnsigned }
impl From<u32> for RvWidth {
    fn from(x: u32) -> Self {
        match x {
            0b000 => Self::Byte,
            0b001 => Self::Half,
            0b010 => Self::Word,
            0b100 => Self::ByteUnsigned,
            0b101 => Self::HalfUnsigned,
            _ => unimplemented!(),
        }
    }
}
impl std::fmt::Display for RvWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Byte => "b",
            Self::Half => "h",
            Self::Word => "w",
            Self::ByteUnsigned => "bu",
            Self::HalfUnsigned => "hu",
        };
        write!(f, "{}", s)
    }
}


/// RV32I branch opcodes.
#[derive(Debug)]
pub enum RvBranchOp { Eq, Ne, Lt, Ge, Ltu, Geu }
impl From<u32> for RvBranchOp {
    fn from(x: u32) -> Self {
        match x {
            0b000 => Self::Eq,
            0b001 => Self::Ne,
            0b010 => unimplemented!(),
            0b011 => unimplemented!(),
            0b100 => Self::Lt,
            0b101 => Self::Ge,
            0b110 => Self::Ltu,
            0b111 => Self::Geu,
            _ => unimplemented!(),
        }
    }
}
impl std::fmt::Display for RvBranchOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Eq => "eq",
            Self::Ne => "ne",
            Self::Lt => "lt",
            Self::Ge => "ge",
            Self::Ltu => "ltu",
            Self::Geu => "geu",
            _ => unimplemented!(),
        };
        write!(f, "{}", s)
    }
}


/// 
#[repr(transparent)]
pub struct Reg(u32);
impl Reg {
    pub fn new(idx: u32) -> Self {
        assert!(idx < 32);
        Self(idx)
    }
    pub fn val(&self) -> u32 { 
        self.0 
    }
}
impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.0)
    }
}


pub enum Instr {
    /// ALU operation
    Op { rd: Reg, rs1: Reg, rs2: Reg, alu_op: RvALUOp },

    /// ALU operation with immediate
    OpImm { rd: Reg, rs1: Reg, simm: i32, alu_op: RvALUOpImm },

    /// Memory load
    Load { rd: Reg, rs1: Reg, simm: i32, width: RvWidth },

    /// Jump-and-link register
    Jalr { rd: Reg, rs1: Reg, simm: i32 },

    /// Add upper immediate to program counter
    AuiPc { rd: Reg, uimm: u32 },

    /// Load upper immediate
    Lui { rd: Reg, uimm: u32 },

    /// Memory store
    Store { rs1: Reg, rs2: Reg, simm: i32, width: RvWidth },

    /// Jump-and-link
    Jal { rd: Reg, simm: i32 },

    /// Conditional branch
    Branch { rs1: Reg, rs2: Reg, simm: i32, brn_op: RvBranchOp },
}
impl std::fmt::Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Op { rd, rs1, rs2, alu_op } => {
                let alu_op = format!("{}", alu_op);
                write!(f, "{:6} {}, {}, {}", alu_op, rd, rs1, rs2)
            },
            Self::OpImm { rd, rs1, simm, alu_op } => {
                let alu_op = format!("{}", alu_op);
                write!(f, "{:6} {}, {}, {}", alu_op, rd, rs1, simm)
            },
            Self::Load { rd, rs1, simm, width } => {
                let inst = format!("l{}", width);
                write!(f, "{:6} {}, {}({})", inst, rd, simm, rs1)
            },
            Self::Jalr { rd, rs1, simm } => {
                write!(f, "{:6} {}, {}({})", "jalr", rd, simm, rs1)
            },
            Self::AuiPc { rd, uimm } => {
                write!(f, "{:6} {}, 0x{:08x}", "auipc", rd, uimm)
            },
            Self::Lui { rd, uimm } => {
                write!(f, "{:6} {}, 0x{:08x}", "lui", rd, uimm)
            },
            Self::Store { rs1, rs2, simm, width } => {
                let inst = format!("s{}", width);
                write!(f, "{:6} {}, {}({})", inst, rs2, simm, rs1)
            },
            Self::Jal { rd, simm } => {
                write!(f, "{:6} {}, {}", "jal", rd, simm)
            },
            Self::Branch { rs1, rs2, simm, brn_op } => {
                let inst = format!("b{}", brn_op);
                write!(f, "{:6} {}, {}, {}", inst, rs1, rs2, simm)
            },
        }
    }
}




/// Representing the RV32I instruction set.
pub struct Rv32;
impl Rv32 {

    // Bitmasks for fixed fields
    const MASK_OP_2:   u32 = 0b0000000_00000_00000_000_00000_1111100;
    const MASK_RD_7:   u32 = 0b0000000_00000_00000_000_11111_0000000;
    const MASK_F3_12:  u32 = 0b0000000_00000_00000_111_00000_0000000;
    const MASK_RS1_15: u32 = 0b0000000_00000_11111_000_00000_0000000;
    const MASK_RS2_20: u32 = 0b0000000_11111_00000_000_00000_0000000;
    const MASK_F7_25:  u32 = 0b1111111_00000_00000_000_00000_0000000;

    // I-type immediate bitmask
    const MASK_I_IMM12_20_31: u32 = 0b1111111_11111_00000_000_00000_0000000;

    // S-type immediate bitmasks
    const MASK_S_IMM5_07_11: u32  = 0b0000000_00000_00000_000_11111_0000000;
    const MASK_S_IMM7_25_31: u32  = 0b1111111_00000_00000_000_00000_0000000;

    // B-type immediate bitmasks
    const MASK_B_IMM1_07_07: u32  = 0b0000000_00000_00000_000_00001_0000000;
    const MASK_B_IMM4_08_11: u32  = 0b0000000_00000_00000_000_11110_0000000;
    const MASK_B_IMM6_25_30: u32  = 0b0111111_00000_00000_000_00000_0000000;
    const MASK_B_IMM1_31_31: u32  = 0b1000000_00000_00000_000_00000_0000000;

    // U-type immediate bitmask
    const MASK_U_IMM20_12_31: u32 = 0b1111111_11111_11111_111_00000_0000000;

    // J-type immediate bitmasks
    const MASK_J_IMM8_12_19: u32  = 0b0000000_00000_11111_111_00000_0000000;
    const MASK_J_IMM1_20_20: u32  = 0b0000000_00001_00000_000_00000_0000000;
    const MASK_J_IMM4_21_24: u32  = 0b0000000_11110_00000_000_00000_0000000;
    const MASK_J_IMM6_25_30: u32  = 0b0111111_00000_00000_000_00000_0000000;
    const MASK_J_IMM1_31_31: u32  = 0b1000000_00000_00000_000_00000_0000000;

    /// Sign-extend some 32-bit number to 'bits'.
    fn sext32(x: u32, bits: u32) -> i32 {
        ((x << (32 - bits)) as i32) >> (32 - bits)
    }

    /// Build an immediate for I-type encodings.
    fn build_i_imm(enc: u32) -> i32 {
        let imm = (enc & Self::MASK_I_IMM12_20_31) >> 20;
        Self::sext32(imm, 12)
    }

    /// Build an immediate for S-type encodings.
    fn build_s_imm(enc: u32) -> i32 {
        let imm = (
               ((enc & Self::MASK_S_IMM5_07_11) >>  7) 
            | (((enc & Self::MASK_S_IMM7_25_31) >> 25) << 5)
        );
        Self::sext32(imm, 12)
    }

    /// Build an immediate for B-type encodings.
    fn build_b_imm(enc: u32) -> i32 {
        let imm = (
               ((enc & Self::MASK_B_IMM4_08_11) >>  8) 
            | (((enc & Self::MASK_B_IMM6_25_30) >> 25) <<  4) 
            | (((enc & Self::MASK_B_IMM1_07_07) >>  7) << 10)
            | (((enc & Self::MASK_B_IMM1_31_31) >> 31) << 11)
        );
        Self::sext32(imm, 12) << 1
    }

    /// Build an immediate for U-type encodings.
    fn build_u_imm(enc: u32) -> u32 {
        enc & Self::MASK_U_IMM20_12_31
    }

    /// Build an immediate for J-type encodings.
    fn build_j_imm(enc: u32) -> i32 {
        let imm = (
               ((enc & Self::MASK_J_IMM4_21_24) >> 21)
            | (((enc & Self::MASK_J_IMM6_25_30) >> 25) << 4)
            | (((enc & Self::MASK_J_IMM1_20_20) >> 20) << 10)
            | (((enc & Self::MASK_J_IMM8_12_19) >> 12) << 11)
            | (((enc & Self::MASK_J_IMM1_31_31) >> 31) << 19)
        );
        Self::sext32(imm, 20) << 1
    }
}


impl InstructionSet for Rv32 {
    type Encoding = u32;
    type Inst     = Instr;

    /// Decode an RV32I instruction.
    fn decode(enc: Self::Encoding) -> Self::Inst {

        // The positions of these fields are always fixed.
        let op  = (enc & Self::MASK_OP_2)   >>  2;
        let rd  = (enc & Self::MASK_RD_7)   >>  7;
        let f3  = (enc & Self::MASK_F3_12)  >> 12;
        let rs1 = (enc & Self::MASK_RS1_15) >> 15;
        let rs2 = (enc & Self::MASK_RS2_20) >> 20;
        let f7  = (enc & Self::MASK_F7_25)  >> 25;

        let rd  = Reg::new(rd);
        let rs1 = Reg::new(rs1);
        let rs2 = Reg::new(rs2);


        match Opcode::from(op) {
            // R-type formats
            Opcode::OP     => {
                let alu_op = RvALUOp::from((f3, f7));
                Instr::Op { rd, rs1, rs2, alu_op }
            },

            // I-type formats
            Opcode::MISC_MEM => unimplemented!(),
            Opcode::SYSTEM   => unimplemented!(),
            Opcode::OP_IMM   => {
                let simm   = Self::build_i_imm(enc);
                let alu_op = RvALUOpImm::from((f3, f7));
                Instr::OpImm { rd, rs1, simm, alu_op }
            },
            Opcode::JALR     => {
                let simm   = Self::build_i_imm(enc);
                Instr::Jalr { rd, rs1, simm }
            },
            Opcode::LOAD => {
                let simm   = Self::build_i_imm(enc);
                let width  = RvWidth::from(f3);
                Instr::Load { rd, rs1, simm, width }
            },

            // S-type formats
            Opcode::STORE  => {
                let simm   = Self::build_s_imm(enc);
                let width  = RvWidth::from(f3);
                Instr::Store { rs1, rs2, simm, width }
            },

            // B-type formats
            Opcode::BRANCH => {
                let simm   = Self::build_b_imm(enc);
                let brn_op = RvBranchOp::from(f3);
                Instr::Branch { rs1, rs2, simm, brn_op }
            },

            // U-type formats
            Opcode::AUIPC  => {
                let uimm   = Self::build_u_imm(enc);
                Instr::AuiPc { rd, uimm }
            },
            Opcode::LUI  => {
                let uimm   = Self::build_u_imm(enc);
                Instr::Lui { rd, uimm }
            },

            // J-type formats
            Opcode::JAL    => {
                let simm  = Self::build_j_imm(enc);
                Instr::Jal { rd, simm }
            },
            _ => unimplemented!(),
        }
    }
}

pub enum Rv32Reg {
    Zero,
    Gpr(usize),
}
pub enum Rv32Mem {
}

pub struct Rv32State {
    gpr: [u32; 32],
}
impl ArchitecturalState for Rv32State {
    type RegType = Rv32Reg;
    type MemType = Rv32Mem;
}





