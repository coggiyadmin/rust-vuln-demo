// SAFE — GCM (conceptual stub; no weak mode).
pub fn gcm_encrypt(_key: &[u8], data: &[u8]) -> Vec<u8> {
    data.to_vec() // placeholder: production uses aes-gcm crate
}
