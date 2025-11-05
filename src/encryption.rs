use base64::engine::general_purpose;
use base64::prelude::*;
use hmac::{Hmac, Mac};
use rand::{Rng, RngCore, distributions::Alphanumeric};
use sha2::Sha256;

pub fn gen_token(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

pub fn gen_nonce() -> String {
    let mut n = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut n);
    general_purpose::STANDARD.encode(&n)
}

pub fn compute_hmac(psk: &[u8], nonce_b64: &str, client_id: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let nonce = BASE64_STANDARD.decode(nonce_b64).expect("nonce decode");
    let mut mac = HmacSha256::new_from_slice(psk).expect("HMAC can take key of any size");
    mac.update(&nonce);
    mac.update(client_id.as_bytes());
    let tag = mac.finalize().into_bytes();
    base64::engine::general_purpose::STANDARD.encode(tag)
}
