
use crate::ir::prim::*;

/// A token for a storage location referenced by the IR.
///
/// When lifting guest code into intermediate code, an [IRLoc] represents
/// some storage location which is either used by an [IROp] or defined by
/// an [IRInst].
///
/// ## Constants
/// During lifting, some values used during computations are encoded along
/// with guest instructions. 
///
/// ## Virtual Registers
/// Within an [IRBlock], each IR instruction defines a unique virtual register
/// with it's result.
///
/// ## Guest Registers
/// A type implementing [GuestRegister] stands for a storage location outside
/// of the virtual machine.
///
#[derive(Clone, Copy)]
pub enum IRLoc {
    None,
    /// A constant value.
    Const(u32, IRWidth),
    /// A virtual register.
    VirtualReg(IRVirtualReg, IRWidth),
}

/// State used for tracking virtual register allocations within an [IRBlock].
pub struct IRBlockStorage {
    /// The index of the next virtual register to-be-allocated.
    next_reg: IRVirtualReg,
}
impl IRBlockStorage {
    pub fn new() -> Self {
        Self {
            next_reg: IRVirtualReg(0),
        }
    }
    /// Allocate (define) a new virtual register.
    pub fn def_reg(&mut self, w: IRWidth) -> IRLoc {
        let loc = IRLoc::VirtualReg(self.next_reg, w);
        self.next_reg.inc();
        loc
    }
    pub fn def_const(&mut self, c: u32, w: IRWidth) -> IRLoc {
        IRLoc::Const(c, w)
    }
}




