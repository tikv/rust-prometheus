use std::hash::{BuildHasherDefault, Hasher};

/// Inspired by nohash-hasher, but we avoid the crate dependency because it's in public archive.
#[derive(Copy, Clone, Debug, Default)]
pub struct NoHashHasher(u64);

pub type BuildNoHashHasher = BuildHasherDefault<NoHashHasher>;

impl Hasher for NoHashHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("Invalid use of NoHashHasher");
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}
