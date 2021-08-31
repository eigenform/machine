
use mach_zoo::ir::prim::*;
use mach_zoo::ir::block::*;
use mach_zoo::guest::*;

#[derive(Copy, Clone)]
pub enum MyReg { A, B, C, D }
impl GuestRegister for MyReg {
    fn width(&self) -> IRWidth {
        IRWidth::U32
    }
}

pub struct MyState {
}
impl GuestMachine for MyState {
    type Register = MyReg;
}


#[test]
fn it_works() {
    let mut block = IRBlock::<MyState>::new();

    block.guest_inst();
    let x = block.def_const(0x1, IRWidth::U8);
    let y = block.ldreg(MyReg::A);
    let z = block.add(x, y, IRWidth::U8);

    block.guest_inst();
    let x = block.def_const(0xff, IRWidth::U8);
    let y = block.ldreg(MyReg::A);
    let z = block.add(x, y, IRWidth::U8);

    let mut vmstate = IRRegisters::new_allocate(10);
    //let mut vmstate = IRRegisters::from_block(&block);
}

