
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



#[cfg(test)]
mod test {
    use crate::memory::*;
    use crate::memory::cache::*;

    #[test]
    fn cache_insert() {
        let mut ram:   NaiveRAM<0x0010_0000>    = NaiveRAM::new();
        let mut cache: SetAssocCache<64, 64, 4> = SetAssocCache::new();

        for (idx, addr) in (0x0000_0000..0x0000_4000usize)
            .step_by(64).enumerate() 
        {
            let data = [idx as u8; 64];
            ram.write_bytes(addr, &data);
        }

        for (idx, addr) in (0x0000_0000..0x0000_4000usize)
            .step_by(64).enumerate() 
        {
            if let Some(tag) = cache.snoop_tag(addr) {
                println!("Tag for {:08x}: {:x?}", addr, tag);
            } else {
                println!("No tag for {:08x}", addr);
            }
        }




        //let mut cache: NaiveCache<64, 64, 4> = NaiveCache::new();

        //for addr in (0x0000_0000u32..=0x0010_0000u32).step_by(64) {
        //    let set = NaiveCache::<64, 64, 4>::get_set_idx(addr);
        //    let tag = NaiveCache::<64, 64, 4>::get_tag(addr);
        //    let off = NaiveCache::<64, 64, 4>::get_offset(addr);
        //    println!("{:08x}: set={:02} tag={:05x} off={:02x}", addr, set,tag,off);

        //}

    }
}

