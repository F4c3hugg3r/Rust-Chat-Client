use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::Rng;
use rand::rng;

// Generiert einen zufälligen, URL-sicheren Base64-Token.
// Die Länge des Tokens in Bytes kann angegeben werden
pub fn generate_secure_token(len: usize) -> String {
    let mut bytes = vec![0u8; len];
    rng().fill(&mut bytes[..]);
    URL_SAFE_NO_PAD.encode(&bytes)
}
