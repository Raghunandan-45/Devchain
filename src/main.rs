mod core_types;
mod gamification;
mod challenges;
mod p2p;
mod cli;
mod vm;
mod app_state; // <-- Added new module

use crate::app_state::AppState; // <-- Corrected import
use crate::p2p::P2PService;
use crate::cli::run_cli;
use std::fs::File;
use std::io::BufReader;

const DB_PATH: &str = "devchain_state.json";
const CURRENT_DEVELOPER_ID: &str = "dev_polyglot_001";

fn load_app_state() -> AppState {
    match File::open(DB_PATH) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(state) => {
                    println!("[INFO] Application state loaded from {}.", DB_PATH);
                    state
                },
                Err(_) => {
                    println!("[WARN] Could not parse state file. Starting fresh.");
                    AppState::new()
                }
            }
        },
        Err(_) => {
            println!("[INFO] No existing state file found. Starting fresh.");
            AppState::new()
        }
    }
}

fn save_app_state(state: &AppState) {
    let file = File::create(DB_PATH).expect("Could not create state file.");
    serde_json::to_writer_pretty(file, state).expect("Could not write state to file.");
    println!("\n[INFO] Application state saved to {}.", DB_PATH);
}

#[tokio::main]
async fn main() {
    println!("========================================");
    println!(" Initializing DevChain Node...");
    println!("========================================");

    let mut app_state = load_app_state();
    
    app_state.profiles.entry(CURRENT_DEVELOPER_ID.to_string())
        .or_insert_with(|| crate::gamification::profile::DeveloperProfile::new(CURRENT_DEVELOPER_ID));

    let p2p_service = P2PService::new().await;
    
    println!("\n[INFO] DevChain Node is running for developer: {}", CURRENT_DEVELOPER_ID);

    run_cli(&mut app_state, CURRENT_DEVELOPER_ID, &p2p_service).await;

    save_app_state(&app_state);
}