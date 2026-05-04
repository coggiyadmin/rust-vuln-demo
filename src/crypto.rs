use std::collections::HashMap;

// CWE-327: weak/broken cryptographic algorithms
pub fn hash_password_md5(password: &str) -> String {
    // MD5 is cryptographically broken — CWE-327
    format!("{:x}", md5::compute(password))
}

pub fn hash_password_sha1(password: &str) -> String {
    use sha1::{Sha1, Digest};
    // SHA1 is deprecated for password hashing — CWE-327
    let mut hasher = Sha1::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

// CWE-330: predictable "random" token using timestamp only
pub fn generate_token(user_id: u64) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // CWE-330: not cryptographically random, easily predicted
    format!("{:x}{:x}", user_id, ts)
}

// CWE-326: insufficient key size — 512-bit RSA
pub fn weak_rsa_key_size() -> usize {
    512  // CWE-326: RSA-512 is trivially factorable
}

// CWE-321: hardcoded encryption key
pub fn encrypt_data(data: &[u8]) -> Vec<u8> {
    let key = b"0123456789abcdef";                     // CWE-321: hardcoded AES key
    data.iter().zip(key.iter().cycle())
        .map(|(d, k)| d ^ k)                          // XOR with static key — not real AES
        .collect()
}

// CWE-338: seeding PRNG with predictable value
pub fn insecure_random(seed: u64) -> u64 {
    // LCG with fixed constants — predictable sequence
    seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407)
}

// CWE-347: JWT decoded without signature verification
pub fn decode_jwt_unsafe(token: &str) -> HashMap<String, String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 { return HashMap::new(); }
    // CWE-347: payload decoded but signature never checked
    let payload = base64::decode(parts[1]).unwrap_or_default();
    serde_json::from_slice(&payload).unwrap_or_default()
}
