use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
pub fn decode_token(token: &str, secret: &str) -> Result<serde_json::Value, jsonwebtoken::errors::Error> {
    decode::<serde_json::Value>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::new(Algorithm::HS256)).map(|d| d.claims)
}
