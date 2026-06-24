pub fn api_key() -> String { std::env::var("API_KEY").unwrap_or_default() }
