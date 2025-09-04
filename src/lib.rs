/* src/lib.rs */

use wasm_bindgen::prelude::*;
mod pow;

#[wasm_bindgen]
pub fn solve_pow_challenge(challenge: &str) -> Option<String> {
    pow::solve_challenge(challenge)
}

#[wasm_bindgen]
pub fn solve_pow_challenge_safe(challenge: &str) -> String {
    match pow::solve_challenge(challenge) {
        Some(solution) => solution,
        None => "FAILED".to_string(),
    }
}
