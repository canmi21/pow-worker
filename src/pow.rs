/* src/pow.rs */

use hex;
use sha2::{Digest, Sha256};

const CHALLENGE_DIFFICULTY: &str = "00000";

/// Core validation logic for a PoW solution.
fn validate_solution(challenge: &str, nonce: &str) -> bool {
    let combined_string = format!("{}{}", challenge, nonce);
    let mut hasher = Sha256::new();
    hasher.update(combined_string.as_bytes());
    let hash = hasher.finalize();
    // Convert hash to a hex string and check if it starts with the required difficulty
    let hash_hex = hex::encode(hash);
    hash_hex.starts_with(CHALLENGE_DIFFICULTY)
}

/// Solves a PoW challenge by brute force.
/// Returns the nonce that produces a hash starting with the required difficulty.
pub fn solve_challenge(challenge: &str) -> Option<String> {
    let mut nonce = 0u64;
    loop {
        let nonce_str = nonce.to_string();
        if validate_solution(challenge, &nonce_str) {
            return Some(nonce_str);
        }
        nonce += 1;
        // Prevent infinite loops - give up after 100 million attempts (increased for 5 zeros)
        if nonce > 100_000_000 {
            return None;
        }
    }
}
