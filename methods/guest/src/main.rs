use recursive_lib::{acc_cubic, update_input_hash, verify_proof, JournalState, ProverInput};
use risc0_zkvm::guest::env;

pub fn main() {
    let input: ProverInput = env::read();

    let (private_value, verified_image_id, public_input_hash) = match &input.prev_journal {
        Some(journal) => {
            let prev_state = verify_proof(journal);

            assert_eq!(
                prev_state.image_id, input.expected_image_id,
                "Image ID mismatch"
            );

            let new_value = acc_cubic(input.public_value, prev_state.private_value);
            let new_hash =
                update_input_hash(Some(&prev_state.public_input_hash), input.public_value);

            (new_value, prev_state.image_id, new_hash)
        }
        None => {
            let initial_value = acc_cubic(input.public_value, 0);
            let initial_hash = update_input_hash(None, input.public_value);

            (initial_value, input.expected_image_id, initial_hash)
        }
    };

    let state = JournalState {
        private_value,
        image_id: verified_image_id,
        public_input_hash,
    };
    env::commit(&state);
}
