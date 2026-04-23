use std::{
    alloc::{Layout, alloc, dealloc},
    ptr::NonNull,
};

use crate::allocation::block::*;

trait AllocRaw {
    fn alloc<T>(&self, object: T) -> *const T;
}

pub fn alloc_block(size: BlockSize) -> Result<BlockPtr, BlockError> {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, size);

        let ptr = alloc(layout);

        if ptr.is_null() {
            Err(BlockError::OutOfMemory)
        } else {
            Ok(NonNull::new_unchecked(ptr))
        }
    }
}

pub fn dealloc_block(block: &Block) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(block.size, block.size);

        dealloc(block.as_mut_ptr(), layout);
    }
}
