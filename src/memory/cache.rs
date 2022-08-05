

/// Representing a cache line, with some size given by `NBYTES`.
#[derive(Clone, Copy)]
pub struct CacheLine<const NBYTES: usize> { data: [u8; NBYTES] }
impl <const NBYTES: usize> Default for CacheLine<NBYTES> {
    fn default() -> Self {
        Self { data: [0; NBYTES] }
    }
}

/// A tag associating some physical address to a cache line.
#[derive(Clone, Copy, Default, Debug)]
pub struct CacheTag { 
    /// Whether or not this tag is currently valid.
    valid: bool, 
    /// Whether or not this cache line has been modified.
    dirty: bool,
    /// The tag data (typically the high bits in a physical address).
    tag: usize,
}

/// A set of [CacheTag] elements.
#[derive(Clone, Copy)]
pub struct TagSet<const NWAY: usize> { tags: [CacheTag; NWAY] }
impl <const NWAY: usize> TagSet<NWAY> {
    pub fn new() -> Self {
        Self { tags: [CacheTag::default(); NWAY] }
    }

    /// Given a target tag `ttag`, find a matching tag which is also valid.
    /// Returns a tuple `(usize, &CacheTag)` with the index/way and tag.
    pub fn find_match(&self, ttag: usize) -> Option<(usize, &CacheTag)> {
        self.tags.iter().enumerate().find(|(idx,e)| e.valid && e.tag == ttag)
    }
}

/// A set of [CacheLine] elements.
#[derive(Clone, Copy)]
pub struct CacheSet<const NBYTES: usize, const NWAY: usize> {
    lines: [CacheLine<NBYTES>; NWAY]
}
impl <const NBYTES: usize, const NWAY: usize> CacheSet<NBYTES, NWAY> {
    pub fn new() -> Self {
        Self { lines: [CacheLine::default(); NWAY], }
    }
}

//pub enum CacheResult<const NBYTES: usize> {
//    /// A read transaction completed successfully.
//    Hit([u8; NBYTES]),
//    /// A read transaction resulted in a cache miss.
//    Miss,
//    /// A fill transaction completed successfully.
//    FillOk,
//    /// An error occured for some fill transaction.
//    FillErr,
//    /// A write transaction completed successfully.
//    WriteOk,
//    /// An error occured for some write transaction.
//    WriteErr,
//}

/// A naive model of a set-associative cache.
///
/// Instances of this type are parameterized by some constant values:
///
/// - `NBYTES` is the number of bytes in a cache line
/// - `NSET` is the number of sets
/// - `NWAY` is the number of ways
///
pub struct SetAssocCache
    <const NBYTES: usize, const NSET: usize, const NWAY: usize>
{
    /// Cache tag storage.
    tags: [ TagSet<NWAY>; NSET ],
    /// Cache data storage.
    sets: [ CacheSet<NBYTES, NWAY>; NSET ],
}
impl <const NBYTES: usize, const NSET: usize, const NWAY: usize>
    SetAssocCache<NBYTES, NSET, NWAY>
{
    pub fn new() -> Self {
        Self {
            tags: [   TagSet::new(); NSET ],
            sets: [ CacheSet::new(); NSET ],
        }
    }

    /// Get the offset for the provided address.
    const fn get_offset_bits(addr: usize) -> usize {
        (addr & ((1 << NBYTES.log2()) - 1))
    }

    /// Get the set index for the provided address.
    const fn get_set_bits(addr: usize) -> usize {
        ( (addr >> NBYTES.log2()) & ((1 << NSET.log2()) - 1) ) 
    }

    /// Get the tag bits for the provided address.
    const fn get_tag_bits(addr: usize) -> usize {
        let bit_idx = NBYTES.log2() + NSET.log2();
        ((addr & !((1 << bit_idx) - 1)) >> bit_idx)
    }

    /// Given an address, try to find the matching entry in the cache.
    /// Returns a tuple with a set, way, and reference to the tag data.
    fn lookup(&self, addr: usize) -> Option<(usize, usize, &CacheTag)> {
        let set = Self::get_set_bits(addr);
        let ttag = Self::get_tag_bits(addr);
        if let Some((way, e)) = self.tags[set].find_match(ttag) {
            Some((set, way, e))
        } else {
            None
        }
    }

    /// Try to read a cache line from the provided address.
    fn read(&self, addr: usize) -> Option<CacheLine<NBYTES>> {
        if let Some((set, way, e)) = self.lookup(addr) {
            Some(self.sets[set].lines[way])
        } else {
            None
        }
    }

    /// Fill a cache entry with the provided data and address. 
    fn fill(&self, addr: usize, data: &[u8; NBYTES]) {
        let set = Self::get_set_bits(addr);
        let ttag = Self::get_tag_bits(addr);
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

        //for (idx, addr) in (0x0000_0000..0x0000_4000usize)
        //    .step_by(64).enumerate() 
        //{
        //    if let Some(tag) = cache.snoop_tag(addr) {
        //        println!("Tag for {:08x}: {:x?}", addr, tag);
        //    } else {
        //        println!("No tag for {:08x}", addr);
        //    }
        //}

    }
}

