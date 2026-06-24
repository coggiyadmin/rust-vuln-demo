// weak_crypto TP — ECB mode (CWE-327).
use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
pub fn ecb_encrypt(key: &[u8; 16], block: &mut [u8; 16]) {
    let c = Aes128::new(GenericArray::from_slice(key));
    c.encrypt_block(GenericArray::from_mut_slice(block)); // SINK CWE-327
}
