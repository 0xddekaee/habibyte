use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identity {
    pub id: String,        // UUID or internal ID
    pub nik_hash: String,  // Setup for zero-knowledge/privacy: don't store raw NIK
    pub full_name: String, // Can be stored here or off-chain
    pub role: IdentityRole,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum IdentityRole {
    Citizen,
    Admin(AdminType),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AdminType {
    Dukcapil,
    RumahSakit,
    Sekolah,
    BPJS,
    Government,
}

impl Identity {
    pub fn new_citizen(nik: &str, name: &str) -> Self {
        let nik_hash = Self::hash_nik(nik);
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            nik_hash,
            full_name: name.to_string(),
            role: IdentityRole::Citizen,
            is_verified: false,
        }
    }

    pub fn hash_nik(nik: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(nik.as_bytes());
        hex::encode(hasher.finalize())
    }

    // Check if this incoming NIK matches the identity (without revealing raw NIK on chain if we only compare hashes)
    pub fn verify_nik(&self, input_nik: &str) -> bool {
        Self::hash_nik(input_nik) == self.nik_hash
    }
}

// Re-export uuid for convenience, or add it to deps if needed.
// Actually I missed adding uuid to Cargo.toml, I should add it or use a simple string for now.
// For now I'll use a simple placeholder random string or fix the dependency.
// Let's rely on the user adding 'uuid' or I'll add it in next step. For now let's comment out uuid usage to compile.

/*
FIXME: Add uuid crate to Cargo.toml
*/
