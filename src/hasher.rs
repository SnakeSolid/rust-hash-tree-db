use std::hash::BuildHasherDefault;
use std::hash::Hasher;

#[derive(Default)]
pub struct TrivialHasher {
    hash: u64,
}

impl Hasher for TrivialHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash = self.hash.to_ne_bytes();
        let chunks = bytes.chunks_exact(8);
        let remainder = chunks.remainder();

        for chunk in chunks {
            for (hash, value) in hash.iter_mut().zip(chunk) {
                *hash ^= value;
            }
        }

        for (hash, value) in hash.iter_mut().zip(remainder) {
            *hash ^= value;
        }

        self.hash = u64::from_ne_bytes(hash);
    }

    fn write_u8(&mut self, i: u8) {
        self.hash ^= i as u64;
    }

    fn write_u16(&mut self, i: u16) {
        self.hash ^= i as u64;
    }

    fn write_u32(&mut self, i: u32) {
        self.hash ^= i as u64;
    }

    fn write_u64(&mut self, i: u64) {
        self.hash ^= i as u64;
    }

    fn write_u128(&mut self, i: u128) {
        self.hash ^= (i >> 0) as u64 ^ (i >> 64) as u64;
    }

    fn write_usize(&mut self, i: usize) {
        self.hash ^= i as u64;
    }

    fn write_i8(&mut self, i: i8) {
        self.hash ^= i as u64;
    }

    fn write_i16(&mut self, i: i16) {
        self.hash ^= i as u64;
    }

    fn write_i32(&mut self, i: i32) {
        self.hash ^= i as u64;
    }

    fn write_i64(&mut self, i: i64) {
        self.hash ^= i as u64;
    }

    fn write_i128(&mut self, i: i128) {
        self.hash ^= (i >> 0) as u64 ^ (i >> 64) as u64;
    }

    fn write_isize(&mut self, i: isize) {
        self.hash ^= i as u64;
    }
}

pub type TrivialHasherBuilder = BuildHasherDefault<TrivialHasher>;
