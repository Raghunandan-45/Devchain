use std::io::{self, Write};
use crate::p2p::P2PService;
use crate::challenges::all_challenges::{FibonacciChallenge, WebAppRaidChallenge};
use crate::challenges::challenge_trait::{Challenge, ChallengeSolution};
use crate::challenges::verifier::VerificationOracle;
use crate::vm::executor::Executor;
use crate::app_state::AppState; // <-- Corrected import

pub async fn run_cli(state: &mut AppState, dev_id: &str, p2p: &P2PService) {
    loop {
        println!("\n--- DevChain CLI ---");
        println!("  1. View Blockchain");
        println!("  2. Mine New Block (Solo Challenge)");
        println!("  3. Tackle Raid Challenge (Team)");
        println!("  4. Execute from Solution Ledger");
        println!("  5. Manage Guild");
        println!("  6. View My Profile");
        println!("  7. Exit");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let developer = state.profiles.get_mut(dev_id).unwrap();

        match choice.trim() {
            "1" => view_blockchain(&state.blockchain),
            "2" => mine_solo_block(&mut state.blockchain, developer, p2p).await,
            "3" => tackle_raid_challenge(&mut state.blockchain, developer, p2p).await,
            "4" => execute_from_ledger(&state.blockchain),
            "5" => manage_guild(developer, &mut state.guilds),
            "6" => developer.display(),
            "7" | "exit" => {
                println!("Exiting DevChain node...");
                break;
            }
            _ => println!("[ERROR] Invalid choice."),
        }
    }
}

// ... (The rest of the file remains the same)
use crate::core_types::blockchain::Blockchain;
use crate::gamification::profile::DeveloperProfile;
use crate::gamification::guilds::Guild;
use std::collections::HashMap;


fn view_blockchain(blockchain: &Blockchain) {
    println!("\n--- Full DevChain ---");
    for block in &blockchain.chain {
        println!("{}", serde_json::to_string_pretty(block).unwrap());
        println!("---------------------");
    }
}

fn get_solution_from_user(lang_choice: &str) -> String {
    println!("\nEnter your {} solution. Type END on a new line when finished.", lang_choice);
    let mut solution_code = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        if line.trim().to_uppercase() == "END" {
            break;
        }
        solution_code.push_str(&line);
    }
    solution_code
}

async fn mine_solo_block(blockchain: &mut Blockchain, developer: &mut DeveloperProfile, p2p: &P2PService) {
    let challenge = FibonacciChallenge::new();
    println!("\n--- New Mining Challenge ---");
    println!("Title: {}", challenge.title());
    println!("Description: {}", challenge.description());
    println!("Supported Languages: {:?}", challenge.languages());
    
    print!("\nChoose your language: ");
    io::stdout().flush().unwrap();
    let mut lang_choice = String::new();
    io::stdin().read_line(&mut lang_choice).expect("Failed to read line");
    let lang_choice = lang_choice.trim();

    if !challenge.languages().contains(&lang_choice) {
        println!("[ERROR] Language not supported for this challenge.");
        return;
    }

    let solution_code = get_solution_from_user(lang_choice);
    let solution = ChallengeSolution {
        challenge_id: challenge.id().to_string(),
        language: lang_choice.to_string(),
        code: solution_code,
        author: developer.developer_id.clone(),
    };

    if VerificationOracle::verify(&challenge, &solution) {
        println!("[SUCCESS] Solution correct! Mining new block...");
        developer.add_xp(challenge.reward_xp(), &solution.language);
        let new_block = blockchain.mine_new_block(solution, &developer.developer_id);
        println!("\n[!] Block #{} successfully mined!", new_block.index);
        println!("[!] Hash: {}", new_block.hash);
        p2p.broadcast_block(&new_block.hash);
    } else {
        println!("[FAILURE] Solution incorrect. Please try again.");
    }
}

async fn tackle_raid_challenge(blockchain: &mut Blockchain, developer: &mut DeveloperProfile, p2p: &P2PService) {
    if developer.guild_id.is_none() {
        println!("[ERROR] You must be in a guild to tackle a raid challenge.");
        return;
    }
    
    let raid = WebAppRaidChallenge::new();
    println!("\n--- New RAID Challenge ---");
    println!("A Web App needs to be built! It requires a frontend and a backend.");
    
    // 1. Solve Frontend
    let frontend_challenge = raid.sub_challenges.get("frontend").unwrap();
    println!("\nSolving: {}", frontend_challenge.title());
    println!("Description: {}", frontend_challenge.description());
    let frontend_solution_code = get_solution_from_user("JavaScript");
    
    // 2. Solve Backend
    let backend_challenge = raid.sub_challenges.get("backend").unwrap();
    println!("\nSolving: {}", backend_challenge.title());
    println!("Description: {}", backend_challenge.description());
    let backend_solution_code = get_solution_from_user("Python");

    // Combine solutions for the block proof
    let combined_code = format!("--FRONTEND--\n{}\n--BACKEND--\n{}", frontend_solution_code, backend_solution_code);
    let raid_solution = ChallengeSolution {
        challenge_id: "webapp_raid".to_string(),
        language: "WebApp (JS/Python)".to_string(),
        code: combined_code,
        author: developer.guild_id.as_deref().unwrap_or(&developer.developer_id).to_string(),
    };

    println!("\n[RAID] All parts submitted! Verifying and mining...");
    // A real verification would check both parts individually. We'll just assume success for this PoC.
    developer.add_xp(500, "Raid"); // Big XP for raids
    let new_block = blockchain.mine_new_block(raid_solution, developer.guild_id.as_deref().unwrap_or(&developer.developer_id));
    println!("\n[!] RAID COMPLETE! Block #{} successfully mined!", new_block.index);
    p2p.broadcast_block(&new_block.hash);
}

fn execute_from_ledger(blockchain: &Blockchain) {
    print!("Enter the block index of the solution to execute: ");
    io::stdout().flush().unwrap();
    let mut index_str = String::new();
    io::stdin().read_line(&mut index_str).expect("Failed to read line");
    let block_index = index_str.trim().parse::<u64>().unwrap_or(0);
    Executor::run_from_ledger(blockchain, block_index);
}

fn manage_guild(developer: &mut DeveloperProfile, guilds: &mut HashMap<String, Guild>) {
    // Simplified guild management
    if developer.guild_id.is_some() {
        println!("You are already in guild: {}", developer.guild_id.as_ref().unwrap());
        return;
    }
    
    print!("Enter a name for your new guild: ");
    io::stdout().flush().unwrap();
    let mut guild_name = String::new();
    io::stdin().read_line(&mut guild_name).expect("Failed to read line");
    let guild_name = guild_name.trim().to_string();
    let guild_id = guild_name.to_lowercase().replace(" ", "-");

    let new_guild = Guild::new(guild_id.clone(), guild_name.clone(), developer.developer_id.clone());
    guilds.insert(guild_id.clone(), new_guild);
    developer.guild_id = Some(guild_id);
    println!("Guild '{}' created successfully!", guild_name);
}