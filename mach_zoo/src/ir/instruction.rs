//! Defines instructions and operations in the IR.

use crate::guest::*;
use crate::ir::prim::*;
use crate::ir::storage::*;

/// Some condition/relation between two values in the IR. 
#[derive(Clone, Copy)]
pub enum IRCond {
    Equal,
    NotEqual,
    GtUnsigned,
    GtEqUnsigned,
    LtUnsigned,
    LtEqUnsigned,
}

/// A primitive operation in the IR.
#[derive(Clone, Copy)]
pub enum IROp<R: GuestRegister> {
    Nop,
    GuestInst,

    /// Load from guest memory.
    Ld(IRLoc, IRWidth),
    /// Store to guest memory. 
    St(IRLoc, IRLoc, IRWidth),

    /// Store to guest register.
    StReg(R, IRLoc),
    /// Load from guest register.
    LdReg(R),

    Add(IRLoc, IRLoc),
    Sub(IRLoc, IRLoc),
    Shl(IRLoc, IRLoc),
    Shr(IRLoc, IRLoc),
    And(IRLoc, IRLoc),
     Or(IRLoc, IRLoc),
    Xor(IRLoc, IRLoc),
    Not(IRLoc),
    IsZero(IRLoc),
    IsNonZero(IRLoc),

    Cmp(IRLoc, IRCond, IRLoc),
}

/// An instruction in the IR.
pub struct IRInst<R: GuestRegister> {
    /// A unique virtual register for the result of this IR instruction.
    dst: IRLoc,
    /// The IR operation performed by this instruction.
    op: IROp<R>,
}
impl <R: GuestRegister> IRInst<R> {
    pub fn new(dst: IRLoc, op: IROp<R>) -> Self {
        Self { dst, op }
    }
}



