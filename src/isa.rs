
pub mod rv32i;


pub trait InstructionSet {
    /// The underlying type for an instruction encoding.
    type Encoding;
    /// The set of unique types of instructions.
    type Inst;

    /// Decode a single instruction.
    fn decode(enc: Self::Encoding) -> Self::Inst;

}

/// Interface to an assembler for the instruction set.
pub trait Assembler: InstructionSet {
    /// Encode an instruction.
    fn encode(inst: Self::Inst) -> Self::Encoding;
}




