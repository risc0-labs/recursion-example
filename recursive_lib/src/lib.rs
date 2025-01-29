use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProverInput {
    pub expected_image_id: [u32; 8],
    pub prev_journal: Option<Vec<u8>>,
    pub public_value: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalState {
    pub private_value: u32,
    pub image_id: [u32; 8],
    pub public_input_hash: [u8; 32],
}

pub fn sha256_hash(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

pub fn acc_cubic(public_value: u32, private_value: u32) -> u32 {
    private_value.wrapping_add(
        public_value
            .wrapping_mul(public_value)
            .wrapping_mul(public_value)
    )
}

pub fn verify_proof(journal: &[u8]) -> JournalState {
    let state: JournalState = bincode::deserialize(journal).unwrap();
    env::verify(state.image_id, journal).unwrap();
    state
}

pub fn update_input_hash(prev_hash: Option<&[u8; 32]>, public_value: u32) -> [u8; 32] {
    let hash_right = sha256_hash(&public_value.to_le_bytes());

    match prev_hash {
        None => {
            let mut bytes = Vec::with_capacity(32);
            bytes.extend_from_slice(&hash_right);
            sha256_hash(&bytes)
        },
        Some(prev_hash) => {
            let mut bytes = Vec::with_capacity(64);
            bytes.extend_from_slice(prev_hash);
            bytes.extend_from_slice(&hash_right);
            sha256_hash(&bytes)
        }
    }
}
