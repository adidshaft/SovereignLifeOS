uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct SovereignManager {}

#[uniffi::export]
impl SovereignManager {
    #[uniffi::constructor]
    pub fn new() -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {})
    }

    pub fn test_connection(&self) -> bool {
        true
    }

    /// Encrypts data using a mock implementation (reverses bytes).
    /// TODO: Replace with Arcium MXE Encryption.
    pub fn encrypt_data(&self, plain_text: String) -> Vec<u8> {
        plain_text.into_bytes().into_iter().rev().collect()
    }


    /// Decrypts data using a mock implementation (reverses bytes).
    pub fn decrypt_data(&self, encrypted_bytes: Vec<u8>) -> String {
        let reversed: Vec<u8> = encrypted_bytes.into_iter().rev().collect();
        String::from_utf8(reversed).unwrap_or_else(|_| "Invalid UTF-8".to_string())
    }

    /// Generates a mock ZK proof for voting.
    /// Returns a JSON string with the proof, identity CID, and encrypted vote.
    pub fn generate_vote_proof(&self, identity_cid: String, vote_choice: bool) -> String {
        let vote_str = if vote_choice { "YES" } else { "NO" };
        // In a real implementation, vote_encrypted would be the result of homomorphic encryption or similar.
        // Here we just return the requested safe string.
        let vote_encrypted = "0x...safe"; 
        
        // Construct JSON manually to avoid dependencies for now.
        format!(
            r#"{{"proof": "arcium_zk_mock_hash", "cid": "{}", "vote_encrypted": "{}"}}"#,
            identity_cid, vote_encrypted
        )
    }
}

#[uniffi::export]
pub fn hello_arcium() -> String {
    "Sovereign Core is Active".to_string()
}
