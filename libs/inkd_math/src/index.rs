/// Layout of a Bubblegum-style Merkle tree used by the protocol.
///
/// `max_depth` controls the absolute capacity of a tree, while `max_buffer_size`
/// controls how many concurrent inserts can be in flight. The defaults are
/// tuned for a low-cost, medium-volume attestation stream (16,384 leaves).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeGeometry {
    pub max_depth: u8,
    pub max_buffer_size: u16,
}

impl TreeGeometry {
    pub const DEFAULT: TreeGeometry = TreeGeometry {
        max_depth: 14,
        max_buffer_size: 64,
    };

    pub fn capacity(&self) -> u64 {
        1u64 << (self.max_depth as u64)
    }

    pub fn fits(&self, index: u64) -> bool {
        index < self.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_capacity_matches_expectation() {
        let g = TreeGeometry::DEFAULT;
        assert_eq!(g.capacity(), 16_384);
    }

    #[test]
    fn out_of_range_index_is_rejected() {
        let g = TreeGeometry::DEFAULT;
        assert!(g.fits(0));
        assert!(g.fits(16_383));
        assert!(!g.fits(16_384));
    }
}
