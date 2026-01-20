use chrono::Utc;
use habibyte_identity::Identity;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
    pub data: Vec<Transaction>,
    pub validator: String, // Public key or ID of the node that validated this
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub transaction_type: TransactionType,
    pub signature: String, // Digital signature from the issuer
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    RegisterIdentity(Identity),     // New identity registration
    UpdateIdentity(String, String), // Update existing identity (ID, NewHash)
    RevokeIdentity(String),         // Revoke ID
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        data: Vec<Transaction>,
        validator: String,
    ) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Self {
            index,
            timestamp,
            previous_hash,
            hash: String::new(),
            data,
            validator,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let serialized_data = serde_json::to_string(&self.data).unwrap_or_default();
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, serialized_data, self.validator
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }
}

pub struct Ledger {
    pub chain: Vec<Block>,
}

impl Ledger {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "0".to_string(), vec![], "GENESIS".to_string());
        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, validator: String) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            transactions,
            validator,
        );
        self.chain.push(new_block);
    }
}
