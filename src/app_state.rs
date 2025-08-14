use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::core_types::blockchain::Blockchain;
use crate::gamification::profile::DeveloperProfile;
use crate::gamification::guilds::Guild;

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub blockchain: Blockchain,
    pub profiles: HashMap<String, DeveloperProfile>,
    pub guilds: HashMap<String, Guild>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            blockchain: Blockchain::new(),
            profiles: HashMap::new(),
            guilds: HashMap::new(),
        }
    }
}