use methods::{RECURSION_EXAMPLE_ELF, RECURSION_EXAMPLE_ID};
use recursive_lib::{JournalState, ProverInput};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let n = 20;
    let prover = default_prover();

    let test_public_values: Vec<u32> = (0..n).map(|i| 100 + i as u32).collect();

    let input = ProverInput {
        expected_image_id: RECURSION_EXAMPLE_ID,
        prev_journal: None,
        public_value: test_public_values[0],
    };

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    println!("Generating Initial Proof (Step 0)");
    let mut last_receipt = prover.prove(env, RECURSION_EXAMPLE_ELF).unwrap().receipt;

    // chain proofs
    for i in 1..n {
        println!("Generating Proof Step {}", i);
        let input = ProverInput {
            expected_image_id: RECURSION_EXAMPLE_ID,
            prev_journal: Some(last_receipt.journal.bytes.clone()),
            public_value: test_public_values[i as usize],
        };

        let env = ExecutorEnv::builder()
            .add_assumption(last_receipt.clone())
            .write(&input)
            .unwrap()
            .build()
            .unwrap();

        let prove_result = prover.prove(env, RECURSION_EXAMPLE_ELF).unwrap();
        last_receipt = prove_result.receipt;

        // Verify the proof
        last_receipt.verify(RECURSION_EXAMPLE_ID).unwrap();

        let state: JournalState = last_receipt.journal.decode().unwrap();
        println!(
            "Step {} Journal - Accumulated Value: {}, Hash: {:?}",
            i, state.private_value, state.public_input_hash
        );
    }
}
