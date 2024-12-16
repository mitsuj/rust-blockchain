use crate::block::Block;
use crate::mining::mine_block;
use crate::transaction::Transaction;
use log::error;

// generating first block/ genesis block
pub fn generate_genesis_block() -> Block {
    Block::new(0, vec![], String::from("0"), 0)
}

// creating a new block linked to last block
pub fn generate_next_block(
    previous_block: &Block,
    transactions: Vec<Transaction>,
    difficulty: u32,
) -> Block {
    let next_index = previous_block.index + 1;
    mine_block(
        next_index,
        transactions,
        previous_block.hash.clone(),
        difficulty,
    )
}

// validate the entire blockchain
pub fn is_chain_valid(chain: &[Block]) -> bool {
    for i in 1..chain.len() {
        let current_block = &chain[i];
        let previous_block = &chain[i - 1];

        let calculated_hash = current_block.calculate_hash();

        if current_block.hash != calculated_hash {
            error!("Invalid hash for block index {}", current_block.index);
            return false;
        }

        if current_block.previous_hash != previous_block.hash {
            error!(
                "Invalid previous hash for block index {}",
                current_block.index
            );
            return false;
        }
    }
    true
}
