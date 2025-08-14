use serde::{Serialize, Deserialize}; // <-- Added Deserialize

#[derive(Serialize, Deserialize, Debug, Clone)] // <-- Added Deserialize
pub struct ChallengeSolution {
    pub challenge_id: String,
    pub language: String,
    pub code: String,
    pub author: String,
}

// ... (rest of the file is unchanged)
pub trait Challenge {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn description(&self) -> &str;
    fn languages(&self) -> Vec<&str>;
    fn get_solution_for(&self, lang: &str) -> Option<String>;
    fn reward_xp(&self) -> u32;
}