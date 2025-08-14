use serde::{Serialize, Deserialize}; // <-- Added Deserialize

#[derive(Serialize, Deserialize, Debug, Clone)] // <-- Added Deserialize
pub struct Guild {
    pub id: String,
    pub name: String,
    pub members: Vec<String>,
}

impl Guild {
    pub fn new(id: String, name: String, founder_id: String) -> Self {
        Guild {
            id,
            name,
            members: vec![founder_id],
        }
    }
}