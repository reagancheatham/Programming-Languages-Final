use core::fmt;
use std::{error::Error, ptr::NonNull};

use crate::allocation::{errors::BlockError, internal};

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block Error: {self}")
    }
}

impl Error for BlockError {}

pub struct Block {
    pub size: BlockSize,
    ptr: BlockPtr,
}

impl Block {
    pub fn new(size: BlockSize) -> Result<Block, BlockError> {
        if !size.is_power_of_two() {
            return Err(BlockError::BadRequest);
        }

        Ok(Block {
            ptr: internal::alloc_block(size)?,
            size,
        })
    }

    pub fn drop(&self) {
        internal::dealloc_block(self);
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }
}