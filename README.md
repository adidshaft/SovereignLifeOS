
# SovereignLifeOS 🛡️📱

**The First Sovereign, Private, and Agentic AI Operating System.**

> "Your Data. Your Model. Your Life. Completely Sovereign."

---

## 💡 The Idea

In an era where personal data is the currency of tech giants, **SovereignLifeOS** reclaims your digital identity. It is a mobile-first Operating System powered by a local, sovereign AI Agent that manages your life—health, finance, identity, and governance—without ever exposing your raw data to a centralized server.

It leverages **Multiparty Execution Environments (MXE)** to allow your AI to compute on your data (like calculating a health score or generating a voting proof) while keeping the data itself encrypted and invisible, even to the AI provider.

## 🌟 Capabilities & Features

### 1. 🆔 Sovereign Identity & Wallet
- **On-Device Key Generation**: Generates Solana keypairs securely on your device using Rust-based cryptography.
- **Secure Persistence**: Stores encrypted secrets in the iOS Keychain.
- **Biometric Access**: FaceID/TouchID required for critical actions.

### 2. 🗳️ Private ZK Voting
- **Zero-Knowledge Proofs**: Prove you are eligible to vote and check a certain criterion (e.g., "Age > 18" or "Identity Verified") without revealing who you are.
- **Anonymous Casting**: Your vote is cast as a cryptographic proof, decoupling your identity from your choice.

### 3. 🏥 Encrypted Health Data vault
- **Client-Side Encryption**: Health data is encrypted locally before it ever leaves your device.
- **Decentralized Storage**: Encrypted blobs are stored on IPFS via **QuickNode**, ensuring censorship resistance and availability without a central database.

### 4. 🧠 Sovereign Core (Rust + Swift)
- **High-Performance Logic**: Critical cryptography and business logic are written in **Rust** for safety and speed.
- **Native Experience**: Seamlessly integrated into a native SwiftUI iOS application.

---

## 🏗️ Tech Stack & Architecture

This project is a hybrid masterpiece, combining low-level systems programming with high-level mobile experiences and decentralized cloud infrastructure.

| Component | Technology | Role |
|-----------|------------|------|
| **Mobile App** | **Swift / SwiftUI** | The beautiful, intuitive interface for the user. Handles Biometrics, Networking, and UI State. |
| **Logic Core** | **Rust** | The brain. Handles Encryption (AES/ChaCha), Key Management, and ZK Proof generation. |
| **Bridge** | **UniFFI** | The magic glue generating seamless bindings between Rust and Swift. |
| **Privacy** | **Arcium (MXE)** | The privacy engine. Allows "Blind Computation" on encrypted data (Roadmap/Mocked). |
| **Storage** | **QuickNode (IPFS)** | Storing encrypted user data blobs immutably and resiliently. |
| **Identity** | **Solana** | The blockchain layer for Identity Management (DID) and Transaction settlement. |

### Architectural Flow

```mermaid
graph TD
    User([👤 User]) <--> UI[📱 iOS App (SwiftUI)]
    
    subgraph "Local Device (Sovereign Enclave)"
        UI <--> |FFI Bridge| Core[🦀 Rust Sovereign Core]
        Core -- "Gen Keys / Proofs" --> KeyStore[(🔒 Keychain)]
    end
    
    subgraph "Decentralized Cloud"
        UI -- "Upload Encrypted Storage" --> IPFS[📦 QuickNode IPFS]
        Core -. "MXE Computation" .-> Arcium[👁️‍🗨️ Arcium Network]
        UI -- "On-Chain Actions" --> Sol[◎ Solana Blockchain]
    end
    
    IPFS <--> |"Encrypted Data"| Arcium
```

---

## 🏃 Mock Example: "A Day in Sovereign Life"

1.  **Morning Check-in**: You log your health metrics (heart rate, mood).
2.  **Encryption**: The **Rust Core** immediately encrypts this data.
3.  **Storage**: The **iOS App** uploads the encrypted blob to **QuickNode IPFS**. You receive a Content ID (CID).
4.  **Voting Time**: A governance proposal asks: *"Should we increase the health budget?"*
5.  **Proof Generation**: You vote "YES". The **Rust Core** generates a ZK Proof: `{"proof": "zk_hash...", "vote": "encrypted_yes"}` using your saved Identity.
6.  **Submission**: This proof is submitted to the chain using your **Solana Wallet**. The network verifies you are a valid citizen without knowing *who* you are.

---


## � Security & "No Lack" Guarantee
This app follows a Zero-Trust, Zero-Leak philosophy. Here is why it is secure:

- **Hardware-Backed Persistence**: We do not store keys in `UserDefaults` or a file. Private keys are stored in the iOS **Secure Enclave (Keychain)**.
- **Biometric Gate**: The app is unusable without **FaceID/TouchID** authentication.
- **Rust Isolation**: Critical crypto operations happen in memory-safe **Rust**, not in high-level Swift.
- **Client-Side Only**: There is no backend server. If we (the devs) disappear, the app still works because it talks directly to public protocols protocol (IPFS/Solana).

---

## 🧪 Mock Scenario: Meet Alice
1.  **Setup**: Alice opens the app. FaceID authenticates her. Rust generates a fresh wallet keypair and saves it to her Keychain.
2.  **Identity**: Alice types "Alice, 1995". The app encrypts this into `0xAb5...` and puts it on IPFS. She gets a CID: `QmXyZ...`.
3.  **Voting**: A vote asks *"Should we fund the park?"*. Alice votes **YES**. The app sends `QmXyZ... + YES` (encrypted) to Arcium. Arcium verifies `QmXyZ` is a valid citizen without revealing "Alice".
4.  **Health**: Alice goes to a new doctor. She unlocks her "Health" module, decrypts her record, and shows the doctor her blood type. She shares the CID, not a file.

---

## ⚖️ For the Judges (Technical Nuances)
-   **FFI Complexity**: We aren't just using an API SDK. We compiled a custom **Rust library** into a native iOS binary (`.xcframework`) using **UniFFI** to ensure performant, safe encryption on mobile.
-   **Multipart IPFS**: We implemented a custom `multipart/form-data` encoder in Swift to handle QuickNode's IPFS requirements without heavy third-party libraries.
-   **Rate Limits**: We implemented smart error handling for RPC rate limits on the Solana Devnet.

---

## 🚀 How to Contribute / Build

### Prerequisites
-   Rust (`cargo`)
-   Xcode 15+
-   QuickNode API Key

### Step-by-Step Build

1.  **Clone the Repo**:
    ```bash
    git clone https://github.com/adidshaft/SovereignLifeOS.git
    cd SovereignLifeOS
    ```

2.  **Compile the Brain (Rust)**:
    *(Critical Step: Generates the iOS framework)*
    ```bash
    cd core
    ./build-ios.sh
    # Wait for "Build Succeeded" message
    ```

3.  **Configure Secrets**:
    -   Create `ios/SovereignLife/SovereignLife/Secrets.plist`.
    -   Add keys: `QUICKNODE_API_KEY`, `SOLANA_RPC_URL`, and `QUICKNODE_IPFS_GATEWAY`.

4.  **Run in Xcode**:
    -   Open `ios/SovereignLife.xcodeproj`.
    -   Select your Simulator (iPhone 15).
    -   Cmd + R.

---

## 🔮 Possible Next Steps
-   **Social Recovery**: Split the private key into 3 shards (Shamir's Secret Sharing) and give them to 3 friends for account recovery.
-   **Zk-Login**: Integrate Google Sign-In via ZK-Proofs (using Arcium) to map email to wallet without doxxing.
-   **Off-Line Mode**: Cache encrypted blobs locally so the app works (read-only) on an airplane.

---

*Built with ❤️, 🦀 Rust, and ⚡ Solana.*
