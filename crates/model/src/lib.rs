use serde::{Serialize, Deserialize};
use Larity_crypto::signatures;

pub type BlockHash = [u8; 32];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub parents: Vec<BlockHash>,
    pub nonce: u64,
    pub timestamp: u64,
    pub state_root: [u8; 32],
    pub tx_root: [u8; 32],
    pub knight_params: KnightParams,
    pub pow_signature: [u8; 64],
    pub snark_proof: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KnightParams {
    // Placeholder for DAGKnight parameters
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub version: u8,
    // Simplified transaction structure
    pub data: Vec<u8>,
    pub signature: [u8; signatures::sphincs::SIGNATURE_BYTES],
    pub zk_proof: Option<[u8; 48]>,
}
