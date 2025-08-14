use crate::challenges::challenge_trait::Challenge;
use std::collections::HashMap;

// --- SOLO CHALLENGE ---
// --- SOLO CHALLENGE ---
pub struct FibonacciChallenge {
    solutions: HashMap<String, String>,
}

impl FibonacciChallenge {
    pub fn new() -> Self {
        let mut solutions = HashMap::new();
        solutions.insert("Python".to_string(), "def fib(n):\n  a, b = 0, 1\n  for _ in range(n):\n    a, b = b, a + b\n  return a".to_string());
        solutions.insert("JavaScript".to_string(), "function fib(n) {\n  let a = 0, b = 1;\n  for (let i = 0; i < n; i++) {\n    [a, b] = [b, a + b];\n  }\n  return a;\n}".to_string());
        solutions.insert("Rust".to_string(), "fn fib(n: u32) -> u64 {\n  let mut a = 0;\n  let mut b = 1;\n  for _ in 0..n {\n    let temp = a;\n    a = b;\n    b = temp + b;\n  }\n  a\n}".to_string());
        solutions.insert("Go".to_string(), "func Fib(n uint) uint64 {\n\tvar a, b uint64 = 0, 1\n\tfor i := uint(0); i < n; i++ {\n\t\ta, b = b, a+b\n\t}\n\treturn a\n}".to_string());
        solutions.insert("Java".to_string(), "public static long fib(int n) {\n    long a = 0, b = 1;\n    for (int i = 0; i < n; i++) {\n        long temp = a;\n        a = b;\n        b = temp + b;\n    }\n    return a;\n}".to_string());
        solutions.insert("Cpp".to_string(), "long long fib(int n) {\n    long long a = 0, b = 1;\n    for (int i = 0; i < n; ++i) {\n        long long temp = a;\n        a = b;\n        b = temp + b;\n    }\n    return a;\n}".to_string());
        solutions.insert("CSharp".to_string(), "public static long Fib(int n) {\n    long a = 0, b = 1;\n    for (int i = 0; i < n; i++) {\n        long temp = a;\n        a = b;\n        b = temp + b;\n    }\n    return a;\n}".to_string());
        solutions.insert("Swift".to_string(), "func fib(n: Int) -> UInt64 {\n    var a: UInt64 = 0\n    var b: UInt64 = 1\n    for _ in 0..<n {\n        let temp = a\n        a = b\n        b = temp + b\n    }\n    return a\n}".to_string());
        solutions.insert("Kotlin".to_string(), "fun fib(n: Int): Long {\n    var a = 0L\n    var b = 1L\n    for (i in 0 until n) {\n        val temp = a\n        a = b\n        b = temp + a\n    }\n    return a\n}".to_string());
        solutions.insert("TypeScript".to_string(), "function fib(n: number): number {\n  let a = 0, b = 1;\n  for (let i = 0; i < n; i++) {\n    [a, b] = [b, a + b];\n  }\n  return a;\n}".to_string());
        
        FibonacciChallenge { solutions }
    }
}

impl Challenge for FibonacciChallenge {
    fn id(&self) -> &str { "fibonacci" }
    fn title(&self) -> &str { "Algorithmic Challenge: Fibonacci Number" }
    fn description(&self) -> &str { "Write a function that returns the nth Fibonacci number." }
    fn languages(&self) -> Vec<&str> { self.solutions.keys().map(|s| s.as_str()).collect() }
    fn get_solution_for(&self, lang: &str) -> Option<String> { self.solutions.get(lang).cloned() }
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
    fn reward_xp(&self) -> u32 { 200 }
}