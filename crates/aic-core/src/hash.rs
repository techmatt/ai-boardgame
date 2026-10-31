//! A small, stable, dependency-free hash (64-bit FNV-1a) for config hashes,
//! content hashes and RNG stream ids. Not cryptographic; stable across
//! platforms and Rust versions, which `std::hash` is not.

pub(crate) const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
pub(crate) const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_vectors() {
        assert_eq!(super::fnv1a64(b""), 0xcbf29ce484222325);
        assert_eq!(super::fnv1a64(b"a"), 0xaf63dc4c8601ec8c);
    }
}
