mod block;
mod blockchain;
mod mining;
mod transaction;
use chrono::Utc;
fn main() {
    let mut chain = vec![blockchain::generate_genesis_block()];
    let difficulty = 4;

    let transaction1 = transaction::Transaction::new(
        "sender1".to_string(),
        "receiver1".to_string(),
        10.0,
        Utc::now().timestamp(),
    );
    let transaction2 = transaction::Transaction::new(
        "sender2".to_string(),
        "receiver2".to_string(),
        5.0,
        Utc::now().timestamp(),
    );

    let block1 = blockchain::generate_next_block(&chain[0], vec![transaction1], difficulty);

    chain.push(block1);

    let transaction3 = transaction::Transaction::new(
        "sender3".to_string(),
        "receiver3".to_string(),
        20.0,
        Utc::now().timestamp(),
    );

    let block2 =
        blockchain::generate_next_block(&chain[1], vec![transaction2, transaction3], difficulty);

    chain.push(block2);

    for block in &chain {
        println!("Block: {block:?}")
    }

    println!("Chain is valid: {}", blockchain::is_chain_valid(&chain));

    let mut invalid_block = chain[2].clone();
    invalid_block
        .transactions
        .push(transaction::Transaction::new(
            "sender4".to_string(),
            "receiver4".to_string(),
            30.0,
            Utc::now().timestamp(),
        ));
    chain[2] = invalid_block;

    println!(
        "Chain is valid after tampering: {}",
        blockchain::is_chain_valid(&chain)
    );
}
