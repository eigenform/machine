//! Defines a container for a basic block of intermediate code.

use crate::guest::*;
use crate::ir::instruction::*;
use crate::ir::prim::*;
use crate::ir::storage::*;

/// A block of straight-line intermediate code.
pub struct IRBlock<G: GuestMachine> {
    /// The set of IR instructions in this block.
    inst: Vec<IRInst<G::Register>>,
    /// Book-keeping for virtual register allocation in this block.
    storage: IRBlockStorage,
}
impl <G: GuestMachine> IRBlock<G> {
    pub fn new() -> Self {
        Self {
            inst: Vec::new(),
            storage: IRBlockStorage::new(),
        }
    }
}
impl <G: GuestMachine> IRBlock<G> {
    /// Append an IR instruction to the block.
    pub fn push(&mut self, inst: IRInst<G::Register>) {
        self.inst.push(inst)
    }
    /// Bind a new constant to use in the scope of this block.
    pub fn def_const(&mut self, c: u32, w: IRWidth) -> IRLoc {
        self.storage.def_const(c, w)
    }
    /// Mark the beginning of a lifted guest instruction.
    pub fn guest_inst(&mut self) {
        self.push(IRInst::new(IRLoc::None, IROp::GuestInst));
    }
}

macro_rules! decl_lift_inst_2in_1out { 
    ($name:ident, $enum:ident) => {
        pub fn $name(&mut self, x: IRLoc, y: IRLoc, w: IRWidth) -> IRLoc {
            let res = self.storage.def_reg(w);
            self.push(IRInst::new(res, IROp::$enum(x,y)));
            res
        }
    }
}

impl <G: GuestMachine> IRBlock<G> {
    pub fn ld(&mut self, x: IRLoc, w: IRWidth) -> IRLoc {
        let res = self.storage.def_reg(w);
        self.push(IRInst::new(res, IROp::Ld(x, w)));
        res
    }
    pub fn st(&mut self, x: IRLoc, y: IRLoc, w: IRWidth) {
        self.push(IRInst::new(IRLoc::None, IROp::St(x, y, w)));
    }
    pub fn ldreg(&mut self, r: G::Register) -> IRLoc {
        let res = self.storage.def_reg(r.width());
        self.push(IRInst::new(res, IROp::LdReg(r)));
        res
    }
    pub fn streg(&mut self, r: G::Register, x: IRLoc) {
        self.push(IRInst::new(IRLoc::None, IROp::StReg(r, x)));
    }
    pub fn cmp(&mut self, x: IRLoc, c: IRCond, y: IRLoc) -> IRLoc {
        let res = self.storage.def_reg(IRWidth::U1);
        self.push(IRInst::new(res, IROp::Cmp(x, c, y)));
        res
    }

    decl_lift_inst_2in_1out!(add, Add);
    decl_lift_inst_2in_1out!(sub, Sub);
    decl_lift_inst_2in_1out!(shl, Shl);
    decl_lift_inst_2in_1out!(shr, Shr);
    decl_lift_inst_2in_1out!(and, And);
    decl_lift_inst_2in_1out!(or, Or);
    decl_lift_inst_2in_1out!(xor, Xor);

    pub fn not(&mut self, x: IRLoc) -> IRLoc {
        let res = self.storage.def_reg(IRWidth::U1);
        self.push(IRInst::new(res, IROp::Not(x)));
        res
    }
    pub fn is_zero(&mut self, x: IRLoc) -> IRLoc {
        let res = self.storage.def_reg(IRWidth::U1);
        self.push(IRInst::new(res, IROp::IsZero(x)));
        res
    }
    pub fn is_nonzero(&mut self, x: IRLoc) -> IRLoc {
        let res = self.storage.def_reg(IRWidth::U1);
        self.push(IRInst::new(res, IROp::IsNonZero(x)));
        res
    }
}

