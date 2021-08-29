
/// Implemented on a type representing a set of concrete storage locations
/// (typically, special and general-purpose registers) on the guest machine.
pub trait GuestRegister 
{
}

/// A unique identifier for a virtual register, unique to an [IRBlock].
#[derive(Clone, Copy)]
pub struct VirtualRegIdx(pub u32);
impl VirtualRegIdx {
    pub fn inc(&mut self) { 
        self.0 += 1 
    }
}

/// A token for a storage location referenced by the IR.
#[derive(Clone, Copy)]
pub enum IRLoc<R: GuestRegister> {
    /// A constant value.
    Const(u32),
    /// A virtual register.
    VirtualReg(VirtualRegIdx),
    /// A guest register.
    GuestReg(R),
}

/// A primitive operation in the IR.
#[derive(Clone, Copy)]
pub enum IROp<R: GuestRegister> {
    /// Load from guest memory.
    Ld(IRLoc<R>),
    /// Store to guest memory. 
    St(IRLoc<R>, IRLoc<R>),

    /// Store to guest register.
    StReg(IRLoc<R>, IRLoc<R>),
    /// Load from guest register.
    LdReg(IRLoc<R>),

    Add(IRLoc<R>, IRLoc<R>),
    Sub(IRLoc<R>, IRLoc<R>),
    Shl(IRLoc<R>, IRLoc<R>),
    Shr(IRLoc<R>, IRLoc<R>),
    And(IRLoc<R>, IRLoc<R>),
     Or(IRLoc<R>, IRLoc<R>),
    Xor(IRLoc<R>, IRLoc<R>),
    Not(IRLoc<R>),
}

/// An instruction in the IR.
pub struct IRInst<R: GuestRegister> {
    /// A unique virtual register for the result of this IR instruction.
    dst: VirtualRegIdx,
    /// The IR operation performed by this instruction.
    op: IROp<R>,
}

/// State used for tracking virtual register allocations within an [IRBlock].
pub struct IRBlockStorage<R: GuestRegister> {
    /// The index of the next virtual register to-be-allocated.
    next_reg: VirtualRegIdx,
    _marker: std::marker::PhantomData<R>,
}
impl <R: GuestRegister> IRBlockStorage<R> {
    pub fn new() -> Self {
        Self {
            next_reg: VirtualRegIdx(0),
            _marker: std::marker::PhantomData,
        }
    }
    /// Allocate (define) a new virtual register.
    pub fn def_reg(&mut self) -> IRLoc<R> {
        let loc = IRLoc::VirtualReg(self.next_reg);
        self.next_reg.inc();
        loc
    }
    pub fn def_const(&mut self, c: u32) -> IRLoc<R> {
        IRLoc::Const(c)
    }
}

/// A block of straight-line intermediate code.
pub struct IRBlock<R: GuestRegister> {
    inst: Vec<IRInst<R>>,
    storage: IRBlockStorage<R>,
}
impl <R: GuestRegister> IRBlock<R> {
    /// Add an [IRInst] to the end of this block.
    pub fn push(&mut self, inst: IRInst<R>) {
        self.inst.push(inst)
    }
}
impl <R: GuestRegister> IRBlock<R> {
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
