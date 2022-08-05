
pub mod cache;

/// A naive model of a simple random-access memory. 
pub struct NaiveRAM<const SIZE: usize> {
    data: Box<[u8; SIZE]>,
}
impl <const SIZE: usize> NaiveRAM<SIZE> {
    pub fn new() -> Self {
        Self {
            data: Box::new([0; SIZE]),
        }
    }

    /// Copy data from RAM at offset `off`, into a slice `dst`.
    pub fn read_bytes(&self, off: usize, dst: &mut [u8]) {
        assert!(off + dst.len() < SIZE);
        dst.copy_from_slice(&self.data[off..(off + dst.len())])
    }

    /// Copy data from a slice `src` into RAM, starting at offset `off`.
    pub fn write_bytes(&mut self, off: usize, src: &[u8]) {
        assert!(off + src.len() < SIZE);
        self.data[off..(off + src.len())].copy_from_slice(src)
    }
}




