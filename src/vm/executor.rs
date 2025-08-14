use crate::core_types::blockchain::Blockchain;

pub struct Executor;

impl Executor {
    /// This is a SIMULATED executor for the Solution Ledger.
    /// It finds a solution in the blockchain and pretends to run it.
    pub fn run_from_ledger(blockchain: &Blockchain, block_index: u64) {
        println!("\n--- Solution Ledger Executor ---");
        if let Some(block) = blockchain.chain.get(block_index as usize) {
            println!("[VM] Found solution in Block #{}", block.index);
            println!("[VM] Author: {}", block.proof.author);
            println!("[VM] Language: {}", block.proof.language);
            println!("[VM] Executing code:\n---\n{}\n---", block.proof.code);
            println!("[VM]... Simulation complete. Royalties sent to {}.", block.proof.author);
        } else {
            println!("[VM-ERROR] Block #{} not found in the ledger.", block_index);
        }
    }
}