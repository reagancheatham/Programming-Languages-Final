use crate::allocation::constants;

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