use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
pub fn decode_token(token: &str) -> Result<serde_json::Value, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation();
    decode::<serde_json::Value>(token, &DecodingKey::from_secret(b"x"), &validation).map(|d| d.claims)
}
