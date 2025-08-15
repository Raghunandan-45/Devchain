use crate::challenges::challenge_trait::Challenge;
use std::collections::HashMap;

// --- SOLO CHALLENGE ---
pub struct FibonacciChallenge {
    solutions: HashMap<String, String>,
    tests: HashMap<String, String>,
}

impl FibonacciChallenge {
    pub fn new() -> Self {
        let mut solutions = HashMap::new();
        let mut tests = HashMap::new();

        // Python
        solutions.insert("Python".to_string(), "def solve(n):\n  a, b = 0, 1\n  for _ in range(n):\n    a, b = b, a + b\n  return a".to_string());
        tests.insert("Python".to_string(), "from solution import solve\nassert solve(10) == 55\nassert solve(0) == 0\nprint('Tests passed!')".to_string());
        
        // JavaScript
        solutions.insert("JavaScript".to_string(), "function solve(n) {\n  let a = 0, b = 1;\n  for (let i = 0; i < n; i++) {\n    [a, b] = [b, a + b];\n  }\n  return a;\n}\nmodule.exports = { solve };".to_string());
        tests.insert("JavaScript".to_string(), "const { solve } = require('./solution.js');\nconst assert = require('assert');\nassert.strictEqual(solve(10), 55);\nassert.strictEqual(solve(0), 0);\nconsole.log('Tests passed!');".to_string());

        // Rust
        solutions.insert("Rust".to_string(), "fn solve(n: u32) -> u64 {\n    let mut a: u64 = 0;\n    let mut b: u64 = 1;\n    for _ in 0..n {\n        let temp = a;\n        a = b;\n        b = temp + b;\n    }\n    a\n}".to_string());
        tests.insert("Rust".to_string(), "fn main() {\n    assert_eq!(solve(10), 55);\n    assert_eq!(solve(0), 0);\n    println!(\"Tests passed!\");\n}".to_string());

        // Go
        solutions.insert("Go".to_string(), "package main\n\nfunc solve(n uint) uint64 {\n\tvar a, b uint64 = 0, 1\n\tfor i := uint(0); i < n; i++ {\n\t\ta, b = b, a+b\n\t}\n\treturn a\n}".to_string());
        tests.insert("Go".to_string(), "package main\n\nimport \"fmt\"\n\nfunc main() {\n    if solve(10) != 55 {\n        panic(\"Test failed for n=10\")\n    }\n    if solve(0) != 0 {\n        panic(\"Test failed for n=0\")\n    }\n    fmt.Println(\"Tests passed!\")\n}".to_string());

        // Java
        solutions.insert("Java".to_string(), "class Solution {\n    public static long solve(int n) {\n        long a = 0, b = 1;\n        for (int i = 0; i < n; i++) {\n            long temp = a;\n            a = b;\n            b = temp + b;\n        }\n        return a;\n    }\n}".to_string());
        tests.insert("Java".to_string(), "public class Main {\n    public static void main(String[] args) {\n        assert Solution.solve(10) == 55;\n        assert Solution.solve(0) == 0;\n        System.out.println(\"Tests passed!\");\n    }\n}".to_string());

        // Cpp
        solutions.insert("Cpp".to_string(), "#include <cstdint>\n\nint64_t solve(int n) {\n    int64_t a = 0, b = 1;\n    for (int i = 0; i < n; ++i) {\n        int64_t temp = a;\n        a = b;\n        b = temp + b;\n    }\n    return a;\n}".to_string());
        tests.insert("Cpp".to_string(), "#include <iostream>\n#include <cassert>\n\nint64_t solve(int n);\n\nint main() {\n    assert(solve(10) == 55);\n    assert(solve(0) == 0);\n    std::cout << \"Tests passed!\" << std::endl;\n    return 0;\n}".to_string());

        // CSharp
        solutions.insert("CSharp".to_string(), "public class Solution {\n    public static long Solve(int n) {\n        long a = 0, b = 1;\n        for (int i = 0; i < n; i++) {\n            long temp = a;\n            a = b;\n            b = temp + b;\n        }\n        return a;\n    }\n}".to_string());
        tests.insert("CSharp".to_string(), "using System;\n\npublic class Program {\n    public static void Main(string[] args) {\n        if (Solution.Solve(10) != 55) throw new Exception();\n        if (Solution.Solve(0) != 0) throw new Exception();\n        Console.WriteLine(\"Tests passed!\");\n    }\n}".to_string());

        // Swift
        solutions.insert("Swift".to_string(), "func solve(_ n: Int) -> UInt64 {\n    var a: UInt64 = 0\n    var b: UInt64 = 1\n    for _ in 0..<n {\n        let temp = a\n        a = b\n        b = temp + b\n    }\n    return a\n}".to_string());
        tests.insert("Swift".to_string(), "import Foundation\n\nassert(solve(10) == 55)\nassert(solve(0) == 0)\nprint(\"Tests passed!\")".to_string());

        // Kotlin
        solutions.insert("Kotlin".to_string(), "fun solve(n: Int): Long {\n    var a = 0L\n    var b = 1L\n    for (i in 0 until n) {\n        val temp = a\n        a = b\n        b = temp + a\n    }\n    return a\n}".to_string());
        tests.insert("Kotlin".to_string(), "fun main() {\n    assert(solve(10) == 55L)\n    assert(solve(0) == 0L)\n    println(\"Tests passed!\")\n}".to_string());

        // TypeScript
        solutions.insert("TypeScript".to_string(), "export function solve(n: number): number {\n  let a = 0, b = 1;\n  for (let i = 0; i < n; i++) {\n    [a, b] = [b, a + b];\n  }\n  return a;\n}".to_string());
        tests.insert("TypeScript".to_string(), "import { solve } from './solution';\nimport * as assert from 'assert';\n\nassert.strictEqual(solve(10), 55);\nassert.strictEqual(solve(0), 0);\nconsole.log('Tests passed!');".to_string());
        
        FibonacciChallenge { solutions, tests }
    }
}

impl Challenge for FibonacciChallenge {
    fn id(&self) -> &str { "fibonacci" }
    fn title(&self) -> &str { "Algorithmic Challenge: Fibonacci Number" }
    fn description(&self) -> &str { "Write a function named 'solve' that returns the nth Fibonacci number." }
    fn languages(&self) -> Vec<&str> { self.solutions.keys().map(|s| s.as_str()).collect() }
    fn get_solution_for(&self, lang: &str) -> Option<String> { self.solutions.get(lang).cloned() }
    fn get_test_for(&self, lang: &str) -> Option<String> { self.tests.get(lang).cloned() }
    fn reward_xp(&self) -> u32 { 150 }
}

// --- RAID CHALLENGE ---
pub struct WebAppRaidChallenge {
    pub sub_challenges: HashMap<String, Box<dyn Challenge>>,
}

impl WebAppRaidChallenge {
    pub fn new() -> Self {
        let mut sub_challenges = HashMap::new();
        sub_challenges.insert("frontend".to_string(), Box::new(FrontendRaidPart::new()) as Box<dyn Challenge>);
        sub_challenges.insert("backend".to_string(), Box::new(BackendRaidPart::new()) as Box<dyn Challenge>);
        WebAppRaidChallenge { sub_challenges }
    }
}

// --- RAID SUB-CHALLENGES ---
struct FrontendRaidPart { solutions: HashMap<String, String> }
impl FrontendRaidPart {
    fn new() -> Self {
        let mut solutions = HashMap::new();
        solutions.insert("JavaScript".to_string(), "document.getElementById('btn').innerHTML = 'Hello, DevChain!';".to_string());
        FrontendRaidPart { solutions }
    }
}
impl Challenge for FrontendRaidPart {
    fn id(&self) -> &str { "webapp_frontend" }
    fn title(&self) -> &str { "Raid Part: Frontend" }
    fn description(&self) -> &str { "Write JS to change button text to 'Hello, DevChain!'" }
    fn languages(&self) -> Vec<&str> { vec!["JavaScript"] }
    fn get_solution_for(&self, lang: &str) -> Option<String> { self.solutions.get(lang).cloned() }
    fn get_test_for(&self, _lang: &str) -> Option<String> { None }
    fn reward_xp(&self) -> u32 { 200 }
}

struct BackendRaidPart { solutions: HashMap<String, String> }
impl BackendRaidPart {
    fn new() -> Self {
        let mut solutions = HashMap::new();
        solutions.insert("Python".to_string(), "return {'status': 'ok'}".to_string());
        BackendRaidPart { solutions }
    }
}
impl Challenge for BackendRaidPart {
    fn id(&self) -> &str { "webapp_backend" }
    fn title(&self) -> &str { "Raid Part: Backend" }
    fn description(&self) -> &str { "Write a Python function that returns a JSON status." }
    fn languages(&self) -> Vec<&str> { vec!["Python"] }
    fn get_solution_for(&self, lang: &str) -> Option<String> { self.solutions.get(lang).cloned() }
    fn get_test_for(&self, _lang: &str) -> Option<String> { None }
    fn reward_xp(&self) -> u32 { 200 }
}