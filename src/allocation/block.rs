use core::fmt;
use std::{error::Error, ptr::NonNull};

use crate::allocation::{constants, internal};

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

#[derive(Debug, PartialEq)]
pub enum BlockError {
    BadRequest,
    OutOfMemory,
}

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

pub struct BumpBlock {
    cursor: *const u8,
    limit: *const u8,
    block: Block,
    meta: BlockMeta,
}

impl BumpBlock {
    pub fn new() -> Result<BumpBlock, AllocError> {
        let inner_block = Block::new(constants::BLOCK_SIZE)?;
        let block_ptr = inner_block.as_ptr();

        let block = BumpBlock {
            cursor: unsafe { block_ptr.add(constants::BLOCK_CAPACITY) },
            limit: block_ptr,
            block: inner_block,
            meta: BlockMeta::new(block_ptr),
        };

        Ok(block)
    }

    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let ptr = self.cursor as usize;
        let limit = self.limit as usize;
        let next_ptr = ptr.checked_sub(alloc_size)? & constants::ALLOC_ALIGN_MASK;

        if next_ptr < limit {
            let block_relative_limit =
                unsafe { self.limit.sub(self.block.as_ptr() as usize) } as usize;

            if block_relative_limit > 0 {
                if let Some((cursor, limit)) = self
                    .meta
                    .find_next_available_hole(block_relative_limit, alloc_size)
                {
                    self.cursor = unsafe { self.block.as_ptr().add(cursor) };
                    self.limit = unsafe { self.block.as_ptr().add(limit) };

                    return self.inner_alloc(alloc_size);
                }
            }

            None
        } else {
            self.cursor = next_ptr as *const u8;
            Some(self.cursor)
        }
    }
}

pub struct BlockMeta {
    lines: *mut u8,
}

impl BlockMeta {
    pub fn new(block_ptr: *const u8) -> BlockMeta {
        let mut meta = BlockMeta {
            lines: unsafe { block_ptr.add(constants::LINE_MARK_START) as *mut u8 },
        };

        meta.reset();

        meta
    }

    pub fn find_next_available_hole(
        &self,
        starting_at: usize,
        alloc_size: usize,
    ) -> Option<(usize, usize)> {
        let mut count = 0;
        let starting_line = starting_at / constants::LINE_SIZE;
        let lines_required = (alloc_size + constants::LINE_SIZE - 1) / constants::LINE_SIZE;
        let mut end = starting_line;

        for index in (0..starting_line).rev() {
            let marked = unsafe { *self.lines.add(index) };

            if marked == 0 {
                count += 1;

                if index == 0 && count >= lines_required {
                    let limit = index * constants::LINE_SIZE;
                    let cursor = end * constants::LINE_SIZE;

                    return Some((cursor, limit));
                }
            } else {
                if count > lines_required {
                    let limit = (index + 2) * constants::LINE_SIZE;
                    let cursor = end * constants::LINE_SIZE;

                    return Some((cursor, limit));
                }

                count = 0;
                end = index;
            }
        }

        None
    }

    pub fn reset(&mut self) {
        unsafe {
            for i in 0..constants::LINE_COUNT {
                *self.lines.add(i) = 0;
            }
        }
    }
}

pub struct BlockList {
    head: Option<BumpBlock>,
    overflow: Option<BumpBlock>,
    rest: Vec<BumpBlock>,
}

impl BlockList {
    fn overflow_alloc(&mut self, alloc_size: usize) -> Result<*const u8, AllocError> {
        match self.overflow {
            Some ...,
            None => {
                let mut overflow = BumpBlock::new()?;

                let space = overflow.inner_alloc
            }
        }
    }
}
