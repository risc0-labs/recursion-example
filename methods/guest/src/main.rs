use recursive_lib::{verify_proof, JournalState, ProverInput};
use risc0_zkvm::guest::env;

const INITIAL_VALUE: u32 = 42;
const MULTIPLIER: u32 = 2;

pub fn main() {
    let input: ProverInput = env::read();

    let (value, verified_image_id) = match &input.prev_journal {
        Some(journal) => {
            let prev_state = verify_proof(journal);
            println!(
                "Verified previous proof. Previous value: {}",
                prev_state.value
            );

            assert_eq!(
                prev_state.image_id, input.expected_image_id,
                "Image ID mismatch"
            );

            (prev_state.value * MULTIPLIER, prev_state.image_id)
        }
        None => {
            println!("Starting new chain with INITIAL_VALUE: {}", INITIAL_VALUE);
            (INITIAL_VALUE, input.expected_image_id)
        }
    };

    println!("Committing value: {}", value);

    let state = JournalState {
        value,
        image_id: verified_image_id,
    };
    env::commit(&state);
}
