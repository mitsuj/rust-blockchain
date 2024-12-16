use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: i64,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64, timestamp: i64) -> Self {
        Self {
            sender,
            receiver,
            amount,
            timestamp,
        }
    }
}
