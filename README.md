# DevChain: A Gamified Proof-of-Skill Blockchain

![DevChain Banner](https://placehold.co/1200x300/1a1a1a/ffffff?text=DevChain&font=raleway)

**DevChain is a revolutionary blockchain protocol where the act of "mining" is replaced by solving real-world software development challenges. Instead of Proof-of-Work, we use Proof-of-Skill.**

This repository contains the official Rust implementation of the DevChain node, a proof-of-concept demonstrating a decentralized network where developers earn rewards and build reputation by contributing their coding skills.

---

## 🚀 Core Concepts

At its heart, DevChain is built on three foundational pillars:

1.  **Proof-of-Skill (PoSkl):** Forget burning GPUs on abstract math problems. On DevChain, you mine new blocks by writing code. The network presents challenges, and the first developer to submit a correct, verifiable solution in any of the 10+ supported languages wins the block reward.

2.  **The Solution Ledger:** Every verified solution is permanently recorded on the blockchain, creating a massive, decentralized, and version-controlled software library. Other dApps and smart contracts can call these solutions and pay royalties to the original author, creating a true economy around reusable code.

3.  **Gamification & Collaboration:** Your on-chain identity is a `DeveloperProfile` that levels up, earns XP, and unlocks badges. You can also form on-chain **Guilds** with other developers to tackle complex **Raid Challenges** that require multiple components (e.g., a frontend and a backend) to be built collaboratively.

---

## ✨ Features

This proof-of-concept includes:

* **A Persistent Blockchain:** The state of the chain, profiles, and guilds is saved to `devchain_state.json` and reloaded on startup.
* **Multi-Language Challenge Oracle:** Solve a "Fibonacci" challenge in one of 10 languages (Python, Rust, JavaScript, C++, Go, etc.).
* **Gamified Profiles:** Gain XP and level up your developer profile.
* **Solution Ledger:** Execute a solution from any block in the chain's history.
* **Guilds & Raid Challenges:** Create a developer guild and tackle a multi-part challenge as a team.
* **P2P Networking Placeholder:** A simulated peer-to-peer layer to demonstrate network communication concepts.

---

## 🔧 Getting Started

### Option 1: Download the Latest Release (Recommended)

The easiest way to get started is to download the pre-compiled application for your operating system.

1.  **Go to the [Releases Page](https://github.com/Raghunandan-45/Devchain/releases).**
2.  Under the latest release, find the correct file for your system (e.g., `devchain-windows-amd64.exe` for Windows).
3.  Download the file, and run it from your terminal.

### Option 2: Build from Source

If you are a developer and want to build the project yourself, follow these steps.

#### Prerequisites

* **Rust:** Ensure you have the Rust toolchain installed. If not, get it from [rust-lang.org](https://www.rust-lang.org).
* **WSL (for Windows users):** It is recommended to build and run the project inside the Windows Subsystem for Linux.

### Installation & Running

1.  **Clone the Repository:**
    ```bash
    git clone <your-repo-url>
    cd devchain_full
    ```

2.  **Build the Project:**
    Cargo will fetch all dependencies and compile the source code.
    ```bash
    cargo build
    ```

3.  **Run the Application:**
    Execute the compiled binary to start your DevChain node.
    ```bash
    cargo run
    ```

Upon running, the application will load any existing state from `devchain_state.json` or create a new one if it doesn't exist. You will then be presented with the interactive command-line interface (CLI).

---

## 📂 Project Structure

The codebase is organized into several modules within the `src/` directory, each responsible for a distinct part of the system.

You're right, the text you pasted shows that the formatting for the file tree was lost. This often happens when copying from a web page that has already "rendered" the text.

The best way to copy it is to get the raw source. Here is the exact, raw text for just the Project Structure section. Please copy this block and paste it into the README.md editor on GitHub.

Markdown

## 📂 Project Structure

The codebase is organized into several modules within the `src/` directory, each responsible for a distinct part of the system.

```plaintext
devchain_full/
├── 📄 Cargo.toml

└── 📂 src/
    ├── 📄 main.rs
    ├── 📄 app_state.rs
    ├── 📄 cli.rs
    ├── 📄 p2p.rs

    ├── 📂 core_types/
    │   ├── 📄 mod.rs
    │   ├── 📄 blockchain.rs
    │   └── 📄 transaction.rs

    ├── 📂 gamification/
    │   ├── 📄 mod.rs
    │   ├── 📄 profile.rs
    │   ├── 📄 skills.rs
    │   ├── 📄 badges.rs
    │   └── 📄 guilds.rs

    ├── 📂 challenges/
    │   ├── 📄 mod.rs
    │   ├── 📄 challenge_trait.rs
    │   ├── 📄 verifier.rs
    │   └── 📄 all_challenges.rs

    └── 📂 vm/
        ├── 📄 mod.rs
        └── 📄 executor.rs
```


---

## 🎮 How to Use the CLI

The command-line interface is your portal to the DevChain network.

* **Mine a Block (Solo):** Choose option `2` to tackle the Fibonacci challenge. Select your preferred language and submit your code.
* **Join a Guild:** Choose option `5` to create a new guild. This is required to participate in raids.
* **Tackle a Raid:** Once in a guild, choose option `3`. You'll be prompted to solve both a frontend (JavaScript) and a backend (Python) part of the challenge.
* **Use the Solution Ledger:** Choose option `4` and enter a block number (e.g., `1`) to "execute" the solution stored in that block and see the simulated royalty payment.
* **Check Your Progress:** Choose option `6` to view your `DeveloperProfile`, including your level, XP, badges, and guild status.
* **Exit & Save:** Choose option `7` to exit. The application will automatically save the entire state to `devchain_state.json`.

---

## 🛣️ Future Roadmap

This project is a proof-of-concept. The next steps to move toward a production system include:

* **Real P2P Networking:** Replace the `p2p` placeholder with a full implementation using `libp2p`, including peer discovery, block gossiping, and state synchronization.
* **Secure Verification Oracle:** Replace the simulated verifier with a true sandboxing solution (e.g., using Docker or gVisor) to safely execute untrusted code.
* **Tokenomics:** Implement the DevCoin (DVC) with real wallet balances and transaction capabilities.
* **Advanced Governance:** Build the on-chain governance system for proposing and voting on new challenges.
