use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm,
    Nonce, // Or Aes128Gcm
};
use rand::RngCore;

#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Encryption failed")]
    EncryptionError,
    #[error("Decryption failed")]
    DecryptionError,
    #[error("Storage provider error: {0}")]
    ProviderError(String),
}

pub trait OffChainStorage {
    /// Stores data and returns a reference (e.g. IPFS CID)
    fn store(&self, data: &[u8]) -> Result<String, StorageError>;

    /// Retrieves data by reference
    fn retrieve(&self, reference: &str) -> Result<Vec<u8>, StorageError>;
}

pub struct EncryptedStorage<S: OffChainStorage> {
    provider: S,
    key: [u8; 32],
}

impl<S: OffChainStorage> EncryptedStorage<S> {
    pub fn new(provider: S, key: [u8; 32]) -> Self {
        Self { provider, key }
    }

    pub fn store_encrypted(&self, data: &[u8]) -> Result<String, StorageError> {
        let cipher = Aes256Gcm::new(&self.key.into());
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|_| StorageError::EncryptionError)?;

        // Prepend nonce to ciphertext so we can decrypt later
        let mut final_payload = nonce_bytes.to_vec();
        final_payload.extend(ciphertext);

        self.provider.store(&final_payload)
    }

    pub fn retrieve_decrypted(&self, reference: &str) -> Result<Vec<u8>, StorageError> {
        let encrypted_data = self.provider.retrieve(reference)?;
        if encrypted_data.len() < 12 {
            return Err(StorageError::DecryptionError);
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);
        let cipher = Aes256Gcm::new(&self.key.into());

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| StorageError::DecryptionError)
    }
}
