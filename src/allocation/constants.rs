pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;
pub const LINE_SIZE_BITS: usize = 7;
pub const LINE_SIZE: usize = 1 << LINE_SIZE_BITS;
pub const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;
pub const ALLOC_ALIGN_MASK: usize = !(size_of::<usize>() - 1);

// We need LINE_COUNT number of bytes for marking lines, so the capacity of a block
// is reduced by that number of bytes.
pub const BLOCK_CAPACITY: usize = BLOCK_SIZE - LINE_COUNT;
pub const LINE_MARK_START: usize = BLOCK_CAPACITY;
