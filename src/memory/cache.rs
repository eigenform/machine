

/// Representing a cache line, with some size given by `NBYTES`.
#[derive(Clone, Copy)]
pub struct CacheLine<const NBYTES: usize> { 
    pub data: [u8; NBYTES]
}
impl <const NBYTES: usize> CacheLine<NBYTES> {
    /// Clear/zero-out this cache line.
    pub fn clear(&mut self) { 
        self.data = [0; NBYTES];
    }

    /// Fill the entire cache line.
    pub fn fill(&mut self, data: &[u8; NBYTES]) {
        self.data.copy_from_slice(data);
    }

    /// [Partially] write to the cache line.
    pub fn write(&mut self, off: usize, data: &[u8]) {
        assert!(off + data.len() <= NBYTES);
        self.data[off..off+data.len()].copy_from_slice(data)
    }

}

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
impl CacheTag {
    pub fn is_valid(&self) -> bool { self.valid }
    pub fn is_dirty(&self) -> bool { self.dirty }

    /// Reset the state of this tag.
    pub fn invalidate(&mut self) {
        self.valid = false;
        self.dirty = false;
        self.tag   = 0;
    }
}

/// Interface to some cache replacement state/policy.
pub trait ReplacementPolicy<const NWAY: usize>: Default + Copy { 
    /// Select a tag to replace from this set, returning the way index.
    fn replace(&mut self, set: &[CacheTag; NWAY]) -> usize;
}

#[derive(Clone, Copy)]
pub struct RandomPolicy {
    seed: usize,
    off:  usize,
}
impl Default for RandomPolicy {
    fn default() -> Self { 
        Self {
            seed: unsafe { core::arch::x86_64::_rdtsc() as usize },
            off: 0,
        }
    }
}
impl <const NWAY: usize> ReplacementPolicy<NWAY> for RandomPolicy {
    fn replace(&mut self, set: &[CacheTag; NWAY]) -> usize {
        let res = (self.seed & ((1 << NWAY.log2()) - 1));
        if self.seed == 0 {
            let mut next = self.seed;
            next ^= next >> 12;
            next ^= next << 25;
            next ^= next >> 27;
            next  = next.wrapping_mul(0x2545f4914f6cdd1d);
            self.seed = next;
        } else {
            self.seed = self.seed >> NWAY.log2();
        }
        res
    }
}

/// A naive model of a set-associative cache.
///
/// Instances of this type are parameterized by other types/constants:
///
/// - `NBYTES` is the number of bytes in a cache line
/// - `NSET` is the number of sets
/// - `NWAY` is the number of ways
/// - `P` is some cache replacement policy
///
/// NOTE: It's not actually clear what I'm doing with this at all yet. 
/// Don't know if I want this to be just "behavioral," or if I want to
/// eventually have this model timings in some way. 
///
pub struct SetAssocCache
    <const NBYTES: usize, const NSET: usize, const NWAY: usize, P> where
    P: ReplacementPolicy<NWAY>
{
    /// Cache tag storage.
    tags: [ [ CacheTag; NWAY]; NSET ],
    /// Cache line storage.
    sets: [ [ CacheLine<NBYTES>; NWAY]; NSET ],
    /// State associated with the replacement policy.
    _policy: P,
}
impl <const NBYTES: usize, const NSET: usize, const NWAY: usize, P>
    SetAssocCache<NBYTES, NSET, NWAY, P> where P: ReplacementPolicy<NWAY>
{
    pub fn new() -> Self {
        Self {
            tags: [ [  CacheTag::default(); NWAY]; NSET ],
            sets: [ [ CacheLine::default(); NWAY]; NSET ],
            _policy: P::default()
        }
    }

    /// Invalidate an entry in the cache.
    pub fn invalidate(&mut self, addr: usize) {
        if let Some((tag, line)) = self.snoop_mut_checked(addr) {
            tag.invalidate();
        }
    }

    /// Read an entry from the cache.
    pub fn read(&mut self, addr: usize) -> Option<CacheLine<NBYTES>> {
        if let Some((tag, line)) = self.snoop_mut_checked(addr) {
            Some(*line)
        } else {
            None
        }
    }

    /// Authoritatively fill a cache line with data from a remote memory.
    pub fn fill(&mut self, addr: usize, data: &[u8; NBYTES]) {
        if let Some((tag, line)) = self.snoop_mut_checked(addr) {
        } else {
            let set = Self::get_set_bits(addr);
            let new_tag = Self::get_tag_bits(addr);

            // If there's an invalid entry in this set, use it 
            if let Some((tag, line)) = self.find_invalid_mut(set) {
                tag.valid = true;
                tag.dirty = false;
                tag.tag = new_tag;
                line.fill(&data);
            } 
            // Otherwise, we have to invoke some replacement policy
            else {
                let way = self._policy.replace(&self.tags[set]);
                { 
                    let tag = self.get_tag_mut(set, way);
                    tag.valid = true;
                    tag.dirty = false;
                    tag.tag  = new_tag;
                }
                let line = self.get_line_mut(set, way);
                line.fill(&data);
            }
        }
    }

}

/// These are helper functions for retrieving the set index and tag from a 
/// physical address. I figure we probably only care about users representing 
/// physical addresses with `u32` or `usize`, so assuming `usize` seems fine.
///
/// This also assumes that all physical addresses are associated with the 
/// following scheme for an address of N bits:
/// 
///   address bit N                                          address bit 0
///   v                                                                  v
///   [ remaining high bits (tag bits)     | set index   | byte offset   ]
///   [(N+1 - NSET.log2() - NBYTES.log2()) | NSET.log2() | NBYTES.log2() ]
///
/// This is probably a reasonable assumption, for now. 
///
impl <const NBYTES: usize, const NSET: usize, const NWAY: usize, P>
    SetAssocCache<NBYTES, NSET, NWAY, P> where P: ReplacementPolicy<NWAY>
{
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
}

/// These are all private methods for interacting with the state. 
impl <const NBYTES: usize, const NSET: usize, const NWAY: usize, P>
    SetAssocCache<NBYTES, NSET, NWAY, P> where P: ReplacementPolicy<NWAY>
{

    /// Get a mutable reference to some [CacheTag].
    fn get_tag_mut(&mut self, set: usize, way: usize) -> &mut CacheTag {
        &mut self.tags[set][way]
    }

    /// Get a mutable reference to some [CacheLine].
    fn get_line_mut(&mut self, set: usize, way: usize) 
        -> &mut CacheLine<NBYTES> 
    {
        &mut self.sets[set][way]
    }

    /// Try to find an invalid entry in the given set.
    fn find_invalid_mut(&mut self, set: usize) 
        -> Option<(&mut CacheTag, &mut CacheLine<NBYTES>)> 
    {
        self.tags[set].iter_mut().zip(self.sets[set].iter_mut())
            .find(|(tag, line)| !tag.valid)
    }

    /// If the provided address has a valid entry in the cache, get a mutable 
    /// references to the associated tag and cache line.
    fn snoop_mut_checked(&mut self, addr: usize) 
        -> Option<(&mut CacheTag, &mut CacheLine<NBYTES>)>
    {
        let tgt_set = Self::get_set_bits(addr);
        let tgt_tag = Self::get_tag_bits(addr);

        self.tags[tgt_set].iter_mut().zip(self.sets[tgt_set].iter_mut())
            .find(|(tag, _line)| { tag.valid && tag.tag == tgt_tag })
    }

    /// Invalidate a particular tag.
    fn invalidate_entry(&mut self, set: usize, way: usize) {
        self.tags[set][way].invalidate();
    }

    /// Invalidate all ways in a particular set.
    fn invalidate_set(&mut self, set: usize) {
        for (tag, line) in self.tags[set].iter_mut()
            .zip(self.sets[set].iter_mut()) 
        {
            tag.invalidate();
        }
    }

    /// Invalidate the entire cache.
    fn invalidate_cache(&mut self) {
        for (tags, lines) in self.tags.iter_mut().zip(self.sets.iter_mut()) {
            for tag in tags {
                tag.invalidate();
            }
        }
    }

}


#[cfg(test)]
mod test {
    use crate::memory::*;
    use crate::memory::cache::*;

    #[test]
    fn set_associative_random() {
        let mut ram: NaiveRAM<0x0010_0000> = NaiveRAM::new();
        let mut cache: SetAssocCache<64, 64, 4, RandomPolicy> 
            = SetAssocCache::new();

        for (idx, addr) in (0x0000_0000..0x0000_6000usize)
            .step_by(64).enumerate() 
        {
            let data = [idx as u8; 64];
            ram.write_bytes(addr, &data);
        }

        let ranges = &mut [
            (0x0000_0000..0x0000_4000usize),
            (0x0000_4000..0x0000_6000usize),
            (0x0000_0000..0x0000_4000usize),
        ];

        for r in ranges.iter_mut() {
            for (idx, addr) in r.step_by(64).enumerate() 
            {
                if let Some(line) = cache.read(addr) {
                    println!("Hit {:08x}: {:02x?}", addr, line.data[0]);
                } else {
                    let mut data = [0u8; 64];
                    ram.read_bytes(addr, &mut data);
                    cache.fill(addr, &data);
                    println!("Miss {:08x}: fill {:02x}", addr, data[0]);
                }
            }
        }
    }
}

