use crate::challenges::challenge_trait::{Challenge, ChallengeSolution};
use crate::oracle::docker_verifier::DockerOracle;

pub struct VerificationOracle;

impl VerificationOracle {
    pub fn verify(challenge: &dyn Challenge, solution: &ChallengeSolution) -> bool {
        println!("[VERIFIER] Handing off to Docker Oracle for secure execution...");

        if let Some(test_code) = challenge.get_test_for(&solution.language) {
            DockerOracle::verify(&solution.language, &solution.code, &test_code)
        } else {
            println!("[VERIFIER-WARN] No test cases found for language: {}. Assuming success for this simplified challenge.", solution.language);
            // For simplified raid challenges, we'll just check if the solution is not empty
            !solution.code.trim().is_empty()
        }
    }
}