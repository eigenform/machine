//! Primitive types for representing data in the IR.

/// Implemented on numeric types which are representable in the IR.
pub trait Num: Sized + Copy + Clone + std::fmt::LowerHex {
    const MIN: Self;
    const WIDTH: IRWidth;
    fn as_ptr(&self) -> *const Self { 
        self as *const Self 
    }
    fn as_mut(&mut self) -> *mut Self { 
        self as *mut Self 
    }
    fn to_le(self) -> Self { 
        Self::to_le(self) 
    }
    fn to_be(self) -> Self { 
        Self::to_be(self) 
    }

    /// Convert this type from an [IRBacking]. 
    fn from_backing(val: IRBacking) -> Self;
    /// Convert this type to an [IRBacking].
    fn to_backing(self) -> IRBacking;
    /// Wrap this type into an [IRValue].
    fn to_irvalue(self) -> IRValue;
    /// Return the associated [IRWidth] of this type.
    fn to_irwidth(self) -> IRWidth { 
        Self::WIDTH 
    }

    fn from_le_bytes(bytes: &[u8]) -> Self;
    fn from_be_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> &[u8];
}

macro_rules! impl_num_for { ($utype:ty, $stype:ty, $rtid:ident) => {
    impl Num for $utype {
        const MIN: Self = Self::MIN;
        const WIDTH: IRWidth = IRWidth::$rtid;
        fn to_backing(self) -> IRBacking { 
            self as IRBacking 
        }
        fn from_backing(val: IRBacking) -> Self { 
            val as Self 
        }
        fn to_irvalue(self) -> IRValue {
            IRValue::new(self)
        }
        fn from_le_bytes(bytes: &[u8]) -> Self {
            use std::convert::TryInto;
            Self::from_le_bytes(bytes.try_into().unwrap())
        }
        fn from_be_bytes(bytes: &[u8]) -> Self {
            use std::convert::TryInto;
            Self::from_be_bytes(bytes.try_into().unwrap())
        }
        fn to_bytes(&self) -> &[u8] {
            unsafe { 
                std::slice::from_raw_parts(self.as_ptr() as *const u8,
                    std::mem::size_of::<Self>()
                ) 
            }
        }
    }
}}

impl_num_for!( u8,  i8,  U8);
impl_num_for!(u16, i16, U16);
impl_num_for!(u32, i32, U32);


/// The underlying type for values in the virtual machine.
pub type IRBacking = u32;

/// Wrapper around a concrete value in the IR.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct IRValue(IRBacking);
impl IRValue {
    /// Create a new [IRValue] from some `T` implementing [Num].
    pub fn new<T: Num>(x: T) -> Self { 
        Self(T::to_backing(x)) 
    }
    /// Interpret this value as some `T` implementing [Num].
    pub fn get_as<T: Num>(self) -> T { 
        T::from_backing(self.0) 
    }
    pub fn as_bool(self) -> bool {
        self.0 != 0
    }
    /// Interpret this value as a concrete [u8].
    pub fn as_u8(self) -> u8 { 
        self.get_as::<u8>() 
    }
    /// Interpret this value as a concrete [u16].
    pub fn as_u16(self) -> u16 { 
        self.get_as::<u16>() 
    }
    /// Interpret this value as a concrete [u32].
    pub fn as_u32(self) -> u32 { 
        self.get_as::<u32>() 
    }
}

/// The width/type of some value in the IR.
#[derive(Clone, Copy)]
pub enum IRWidth {
    U1,
    U8, 
    U16, 
    U32,
}

/// A unique identifier for a virtual register.
#[derive(Clone, Copy)]
pub struct IRVirtualReg(pub u32);
impl IRVirtualReg {
    pub fn inc(&mut self) { 
        self.0 += 1 
    }
}

/// A set of virtual registers.
pub struct IRRegisters {
    reg: Vec<IRValue>,
}
impl IRRegisters {
    pub fn new_allocate(size: usize) -> Self {
        Self {
            reg: vec![IRValue(0); size],
        }
    }
    //pub fn from_block(blk: &IRBlock) -> Self {
    //    Self {
    //        reg: vec![IRValue(0); blk.storage.
    //    }
    //}
    pub fn read(&self, idx: IRVirtualReg) -> IRValue {
        self.reg[idx.0 as usize]
    }
    pub fn write(&mut self, idx: IRVirtualReg, val: IRValue) {
        self.reg[idx.0 as usize] = val
    }
}


