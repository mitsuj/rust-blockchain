use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String, nonce: u64) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Self {
            index,
            timestamp,
            data,
            previous_hash,
            nonce,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
