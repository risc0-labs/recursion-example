use recursive_lib::{verify_proof, ProverInput};
use risc0_zkvm::guest::env;

const INITIAL_VALUE: u32 = 42;
const MULTIPLIER: u32 = 2;

pub fn main() {
    let input: ProverInput = env::read();

    let value = match &input.prev_journal {
        Some(journal) => {
            let prev_value = verify_proof(journal, &input.expected_image_id);
            println!("Verified previous proof. Previous value: {}", prev_value);
            let new_value = prev_value * MULTIPLIER;
            new_value
        }
        None => {
            println!("Starting new chain with INITIAL_VALUE: {}", INITIAL_VALUE);
            INITIAL_VALUE
        }
    };

    println!("Committing value: {}", value);
    env::commit(&value);
}
