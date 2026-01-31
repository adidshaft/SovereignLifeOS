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


    /// Generates a new Solana wallet Keypair and returns a JSON string containing
    /// the public key and the secret key.
    pub fn create_wallet(&self) -> String {
        use solana_sdk::signer::{keypair::Keypair, Signer};
        let kp = Keypair::new();
        let pub_key = kp.pubkey().to_string();
        let secret_key = format!("{:?}", kp.to_bytes()); // Serialize as byte array string

        // Return JSON with both keys
        // We use manual formatting or serde_json if we want to be fancy.
        // Since we added serde_json, let's use a simple json! macro or format if we didn't derive Serialize.
        // For simplicity and avoiding extra structs, manual JSON construction is also fine for this small payload,
        // but let's use format! to be consistent with previous methods, or cleaner, use serde_json.
        // However, we didn't add Serialize to a struct. Let's just return a dict.
        
        format!(
            r#"{{"pub_key": "{}", "secret_key": "{}"}}"#,
            pub_key, secret_key
        )
    }

    /// Loads a wallet from a secret key string (byte array format) and returns the public key.
    pub fn load_wallet(&self, secret_key: String) -> String {
        use solana_sdk::signer::{keypair::Keypair, Signer};
        
        // innovative parsing of the byte array string "[1, 2, ...]"
        let trimmed = secret_key.trim_matches(|c| c == '[' || c == ']');
        let bytes: Result<Vec<u8>, _> = trimmed
            .split(',')
            .map(|s| s.trim().parse::<u8>())
            .collect();

        match bytes {
            Ok(key_bytes) => {
                match Keypair::from_bytes(&key_bytes) {
                    Ok(kp) => kp.pubkey().to_string(),
                    Err(_) => "Invalid secret key bytes".to_string(),
                }
            },
            Err(_) => "Invalid secret key format".to_string(),
        }
    }
}

#[uniffi::export]
pub fn hello_arcium() -> String {
    "Sovereign Core is Active".to_string()
}
