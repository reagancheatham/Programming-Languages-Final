use anyhow::Result;
use crate::allocation::block::Block;

mod allocation;

fn main() -> Result<()> {
    let size = 8;
    let block = Block::new(size)?;

    block.drop();
    Ok(())
}
