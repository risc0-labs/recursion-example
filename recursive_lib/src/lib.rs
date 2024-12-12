use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProverInput {
    pub expected_image_id: [u32; 8],
    pub prev_journal: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalState {
    pub value: u32,
    pub image_id: [u32; 8],
}

pub fn verify_proof(journal: &[u8]) -> JournalState {
    let state: JournalState = bincode::deserialize(journal).unwrap();
    env::verify(state.image_id, journal).unwrap();
    state
}
