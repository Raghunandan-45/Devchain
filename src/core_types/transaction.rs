use serde::{Serialize, Deserialize}; // <-- Added Deserialize

#[derive(Serialize, Deserialize, Debug, Clone)] // <-- Added Deserialize
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: f64,
}