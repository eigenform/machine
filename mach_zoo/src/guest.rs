
use crate::ir::prim::*;

/// Implemented on a type representing a set of concrete storage locations
/// (typically, special and general-purpose registers) on the guest machine.
pub trait GuestRegister: Copy + Clone
{
    /// Returns the width of this guest register.
    fn width(&self) -> IRWidth;
}

/// Interface to a guest machine.
pub trait GuestMachine {
    /// Type representing registers for this machine.
    type Register: GuestRegister;
}
