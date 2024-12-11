use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProverInput {
    pub expected_image_id: [u32; 8],
    pub prev_journal: Option<Vec<u8>>,
}

pub fn verify_proof(journal: &[u8], image_id: &[u32; 8]) -> u32 {
    env::verify(*image_id, journal).unwrap();
    bincode::deserialize(journal).unwrap()
}
