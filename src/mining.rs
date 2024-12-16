use crate::block::Block;
use crate::transaction::Transaction;

pub fn mine_block(
    index: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    difficulty: u32,
) -> Block {
    println!("Mining block...");
    let prefix_str = "0".repeat(difficulty as usize);
    let mut nonce = 0;

    loop {
        let block = Block::new(index, transactions.clone(), previous_hash.clone(), nonce);

        if block.hash.starts_with(&prefix_str) {
            println!("Block mined!");
            return block;
        }
        nonce += 1;
    }
}
