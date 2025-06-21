pub mod pow {
    use blake3;

    pub fn solve_pow(input: &[u8], difficulty: u64) -> Option<u64> {
        // Simplified PoW solving
        for nonce in 0..difficulty {
            let mut hasher = blake3::Hasher::new();
            hasher.update(input);
            hasher.update(&nonce.to_le_bytes());
            let hash = hasher.finalize();
            
            if u64::from_le_bytes(hash.as_bytes()[..8].try_into().unwrap()) < difficulty {
                return Some(nonce);
            }
        }
        None
    }
    
    pub fn verify_pow(input: &[u8], nonce: u64, difficulty: u64) -> bool {
        let mut hasher = blake3::Hasher::new();
        hasher.update(input);
        hasher.update(&nonce.to_le_bytes());
        let hash = hasher.finalize();
        
        u64::from_le_bytes(hash.as_bytes()[..8].try_into().unwrap()) < difficulty
    }
}

pub mod signatures {
    use sphincs::sphincs_shake_256s_simple as sphincs;
    
    pub struct Keypair {
        pub public: [u8; sphincs::PUBLIC_KEY_BYTES],
        secret: [u8; sphincs::SECRET_KEY_BYTES],
    }
    
    impl Keypair {
        pub fn new() -> Self {
            let mut rng = rand::thread_rng();
            let (public, secret) = sphincs::keygen(&mut rng);
            Self { public, secret }
        }
        
        pub fn sign(&self, message: &[u8]) -> [u8; sphincs::SIGNATURE_BYTES] {
            sphincs::sign(message, &self.secret)
        }
        
        pub fn verify(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
            sphincs::verify(message, signature, public_key).is_ok()
        }
    }
}
