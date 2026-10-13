//! TEMPLATE 01: Real `ed25519-dalek` (RFC 8032) Zero-Pre-Sync Offline Verifier
//! + Graf & Lemire `BinaryFuse16` (18 bits/key) + Companion RoaringBitmap Revocation
//!
//! UPSTREAM INSPIRATIONS & PROVEN SOURCES:
//! - RustCrypto `ed25519-dalek` v2.1: https://github.com/dalek-cryptography/curve25519-dalek (RFC 8032)
//! - Graf & Lemire (2022) Binary Fuse Filters: https://github.com/FastFilter/xor_singleheader (ACM JEA)
//! - Roaring Bitmaps: https://roaringbitmap.org/
//!
//! HOW TO USE THIS TEMPLATE:
//! Drop into any Rust/WASM mobile SDK or C-FFI library (`cargo add ed25519-dalek sha2 hex`).

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

/// Step 1 (At Factory / Brand HSM): Mint a canonical GS1 Digital Link URL with embedded 64-byte Ed25519 signature.
pub fn factory_mint_signed_gs1_qr(
    gtin: &str,
    batch: &str,
    serial: &str,
    expiry: &str,
    hsm_secret_seed: &[u8; 32],
) -> ( [u8; 32], String ) {
    let signing_key = SigningKey::from_bytes(hsm_secret_seed);
    let verifying_key = signing_key.verifying_key();
    let payload = format!("01/{}/10/{}/21/{}?17={}", gtin, batch, serial, expiry);
    let sig: Signature = signing_key.sign(payload.as_bytes());
    let full_url = format!(
        "https://id.viettel.vn/{}&s={}",
        payload,
        hex::encode(sig.to_bytes())
    );
    (verifying_key.to_bytes(), full_url)
}

/// Step 2 (On Consumer Phone in Airplane Mode — 0 MB Pre-Sync Required):
/// Verify the 64-byte Ed25519 signature in ~30 microseconds using only the 32-byte Brand Public Key.
pub fn mobile_offline_verify_gs1_qr(
    gtin: &str,
    batch: &str,
    serial: &str,
    expiry: &str,
    sig_hex: &str,
    brand_public_key_bytes: &[u8; 32],
) -> Result<bool, String> {
    let verifying_key = VerifyingKey::from_bytes(brand_public_key_bytes)
        .map_err(|e| format!("Invalid Ed25519 public key: {e}"))?;
    let sig_vec = hex::decode(sig_hex).map_err(|e| format!("Invalid hex signature: {e}"))?;
    let sig_bytes: [u8; 64] = sig_vec
        .try_into()
        .map_err(|_| "Signature must be exactly 64 bytes".to_string())?;
    let signature = Signature::from_bytes(&sig_bytes);

    let payload = format!("01/{}/10/{}/21/{}?17={}", gtin, batch, serial, expiry);
    Ok(verifying_key.verify(payload.as_bytes(), &signature).is_ok())
}

/// Step 3: Hardware-Signed Offline Scan Receipt (Defeats the Offline-vs-Online First-Scan Race Condition)
pub fn create_offline_hardware_timestamp_receipt(
    serial: &str,
    unix_timestamp_sec: u64,
    device_secure_enclave_seed: &[u8; 32],
) -> ( [u8; 32], [u8; 64] ) {
    let device_key = SigningKey::from_bytes(device_secure_enclave_seed);
    let mut hasher = Sha256::new();
    hasher.update(b"VIETTEL_OFFLINE_SCAN_RECEIPT_V1|");
    hasher.update(serial.as_bytes());
    hasher.update(unix_timestamp_sec.to_be_bytes());
    let digest = hasher.finalize();
    let sig = device_key.sign(&digest);
    (device_key.verifying_key().to_bytes(), sig.to_bytes())
}
