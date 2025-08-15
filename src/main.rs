mod core_types;
mod gamification;
mod challenges;
mod p2p;
mod cli;
mod vm;
mod app_state;
mod oracle;
mod api; // <-- Add new module

use crate::app_state::AppState;
use crate::p2p::P2PService;
use crate::cli::run_cli;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex}; // <-- Add this

const DB_PATH: &str = "devchain_state.json";
const CURRENT_DEVELOPER_ID: &str = "dev_polyglot_001";

fn load_app_state() -> AppState {
    // ... (this function is unchanged)
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
    // ... (this function is unchanged)
    let file = File::create(DB_PATH).expect("Could not create state file.");
    serde_json::to_writer_pretty(file, state).expect("Could not write state to file.");
    println!("\n[INFO] Application state saved to {}.", DB_PATH);
}

#[tokio::main]
async fn main() {
    println!("========================================");
    println!(" Initializing DevChain Node...");
    println!("========================================");

    let app_state = load_app_state();
    
    // Create a thread-safe, shareable reference to the application state
    let shared_state = Arc::new(Mutex::new(app_state));
    
    // Clone the reference for the API server
    let api_state = shared_state.clone();
    
    // Launch the API server in a separate, non-blocking task
    tokio::spawn(async move {
        api::start_api_server(api_state).await;
    });

    // Ensure our current developer profile exists in the state
    { // Create a new scope to release the lock quickly
        let mut state = shared_state.lock().unwrap();
        state.profiles.entry(CURRENT_DEVELOPER_ID.to_string())
            .or_insert_with(|| crate::gamification::profile::DeveloperProfile::new(CURRENT_DEVELOPER_ID));
    }

    let p2p_service = P2PService::new().await;
    println!("\n[INFO] DevChain Node is running for developer: {}", CURRENT_DEVELOPER_ID);

    // Run the CLI with its own reference to the state
    run_cli(shared_state.clone(), CURRENT_DEVELOPER_ID, &p2p_service).await;

    // Save the final state when the CLI exits
    save_app_state(&shared_state.lock().unwrap());
}
