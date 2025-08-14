use serde::{Serialize, Deserialize}; // <-- Added Deserialize
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::core_types::transaction::Transaction;
use crate::challenges::challenge_trait::ChallengeSolution;

#[derive(Serialize, Deserialize, Debug, Clone)] // <-- Added Deserialize
pub struct Block {
    // ... (rest of the struct is unchanged)
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: ChallengeSolution,
    pub previous_hash: String,
    pub hash: String,
}

// ... (impl Block is unchanged)
impl Block {
    pub fn calculate_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::new();
        let serialized = serde_json::to_string(&block_data).expect("Failed to serialize block");
        
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}


#[derive(Serialize, Deserialize)] // <-- Added derive macro
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

// ... (impl Blockchain is unchanged)
impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_solution = ChallengeSolution {
            challenge_id: "genesis".to_string(),
            language: "genesis".to_string(),
            code: "genesis_proof".to_string(),
            author: "system".to_string(),
        };

        let mut genesis_block = Block {
            index: 0,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions: vec![],
            proof: genesis_solution,
            previous_hash: "0".to_string(),
            hash: String::new(),
        };
        genesis_block.hash = genesis_block.calculate_hash();
        self.chain.push(genesis_block);
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn mine_new_block(&mut self, proof: ChallengeSolution, miner_address: &str) -> &Block {
        let reward_tx = Transaction {
            sender: "0".to_string(), // System reward
            recipient: miner_address.to_string(),
            amount: 100.0, // 100 DevCoin (DVC)
        };
        self.pending_transactions.push(reward_tx);

        let previous_hash = self.last_block().hash.clone();
        let mut new_block = Block {
            index: self.chain.len() as u64,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions: self.pending_transactions.clone(),
            proof,
            previous_hash,
            hash: String::new(),
        };
        new_block.hash = new_block.calculate_hash();
        
        self.pending_transactions = Vec::new();
        self.chain.push(new_block);
        self.chain.last().unwrap()
    }
}