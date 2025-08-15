use std::fs;
use std::process::Command;
use tempfile::Builder;
use uuid::Uuid;

pub struct DockerOracle;

impl DockerOracle {
    pub fn verify(language: &str, solution_code: &str, test_code: &str) -> bool {
        let temp_dir = Builder::new().prefix("devchain-oracle").tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();

        let (dockerfile_content, solution_filename, test_runner_filename) = match language {
            "Python" => (
                "FROM python:3.9-slim\nWORKDIR /app\nCOPY . .\nCMD [\"python\", \"run_tests.py\"]",
                "solution.py", "run_tests.py",
            ),
            "JavaScript" => (
                "FROM node:16-slim\nWORKDIR /app\nCOPY . .\nCMD [\"node\", \"run_tests.js\"]",
                "solution.js", "run_tests.js",
            ),
            "Rust" => (
                "FROM rust:1.79 as builder\nWORKDIR /app\nCOPY . .\nRUN cargo build --release\n\nFROM debian:buster-slim\nCOPY --from=builder /app/target/release/solution .\nCMD [\"./solution\"]",
                "main.rs", "main.rs", // In this setup, test is part of the binary
            ),
            "Go" => (
                "FROM golang:1.22 as builder\nWORKDIR /app\nCOPY . .\nRUN go build -o solution .\n\nFROM gcr.io/distroless/static-debian11\nCOPY --from=builder /app/solution .\nCMD [\"./solution\"]",
                "main.go", "main.go",
            ),
            "Java" => (
                "FROM openjdk:11\nWORKDIR /app\nCOPY . .\nRUN javac Main.java\nCMD [\"java\", \"Main\"]",
                "Main.java", "Main.java",
            ),
            "Cpp" => (
                "FROM gcc:latest\nWORKDIR /app\nCOPY . .\nRUN g++ -o solution main.cpp\nCMD [\"./solution\"]",
                "main.cpp", "main.cpp",
            ),
            "CSharp" => (
                "FROM mcr.microsoft.com/dotnet/sdk:6.0\nWORKDIR /app\nCOPY . .\nRUN dotnet build\nCMD [\"dotnet\", \"run\"]",
                "Program.cs", "Program.cs",
            ),
            "Swift" => (
                "FROM swift:5.10\nWORKDIR /app\nCOPY . .\nRUN swiftc main.swift -o solution\nCMD [\"./solution\"]",
                "main.swift", "main.swift",
            ),
            "Kotlin" => (
                "FROM openjdk:11\nWORKDIR /app\nCOPY . .\nRUN kotlinc Main.kt -include-runtime -d solution.jar\nCMD [\"java\", \"-jar\", \"solution.jar\"]",
                "Main.kt", "Main.kt",
            ),
            "TypeScript" => (
                "FROM node:16-slim\nWORKDIR /app\nRUN npm install -g typescript ts-node\nCOPY . .\nCMD [\"ts-node\", \"run_tests.ts\"]",
                "solution.ts", "run_tests.ts",
            ),
            _ => {
                println!("[ORACLE-ERROR] Unsupported language for Docker verification: {}", language);
                return false;
            }
        };

        // For compiled languages where test is part of the binary, we combine them
        let final_solution_code = if solution_filename == test_runner_filename {
            format!("{}\n\n{}", solution_code, test_code)
        } else {
            solution_code.to_string()
        };

        fs::write(temp_path.join("Dockerfile"), dockerfile_content).expect("Unable to write Dockerfile");
        fs::write(temp_path.join(solution_filename), final_solution_code).expect("Unable to write solution file");
        
        if solution_filename != test_runner_filename {
            fs::write(temp_path.join(test_runner_filename), test_code).expect("Unable to write test runner");
        }

        let image_tag = format!("devchain-test-{}", Uuid::new_v4());

        println!("[ORACLE] Building Docker image for verification...");
        let build_status = Command::new("docker")
            .arg("build").arg("-t").arg(&image_tag).arg(temp_path)
            .status().expect("Failed to execute docker build. Is Docker installed and running?");

        if !build_status.success() {
            println!("[ORACLE-ERROR] Docker image build failed.");
            return false;
        }

        println!("[ORACLE] Running container to execute tests...");
        let run_status = Command::new("docker")
            .arg("run").arg("--rm").arg(&image_tag)
            .status().expect("Failed to execute docker run.");

        println!("[ORACLE] Cleaning up Docker image...");
        Command::new("docker").arg("rmi").arg(&image_tag).status().expect("Failed to remove Docker image.");

        run_status.success()
    }
}