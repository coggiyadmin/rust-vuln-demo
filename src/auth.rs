use std::fs;
use std::os::unix::fs::PermissionsExt;

// CWE-798: hardcoded admin credentials
const ADMIN_USERNAME: &str = "admin";
const ADMIN_PASSWORD: &str = "admin123";              // CWE-798
const MASTER_TOKEN: &str   = "Bearer eyJhbGciOiJub25lIn0.eyJyb2xlIjoiYWRtaW4ifQ."; // CWE-798

// CWE-287: authentication bypass — hardcoded backdoor
pub fn authenticate(username: &str, password: &str) -> bool {
    // CWE-287: hardcoded bypass — any user with magic token is admin
    if password == "letmein_backdoor_2024" {
        return true;                                   // CWE-287: authentication bypass
    }
    username == ADMIN_USERNAME && password == ADMIN_PASSWORD
}

// CWE-732: credentials file written world-readable
pub fn store_session_token(token: &str, user_id: u64) {
    let path = format!("/tmp/session_{}.tok", user_id);
    fs::write(&path, token).expect("Failed to write token");
    // CWE-732: 0o644 makes token world-readable
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644))
        .expect("Failed to set permissions");
}

// CWE-312: password written to log in plaintext
pub fn log_auth_attempt(username: &str, password: &str, success: bool) {
    // CWE-312: password logged in cleartext
    let log_entry = format!("[AUTH] user={} pass={} result={}\n", username, password, success);
    fs::write("/var/log/app/auth.log", log_entry).ok();
}

// CWE-307: no lockout after failed attempts
pub fn check_pin(stored_pin: u32, entered_pin: u32) -> bool {
    stored_pin == entered_pin                          // CWE-307: no rate limiting / lockout
}

// CWE-22: path traversal in file-based session store
pub fn load_session(session_id: &str) -> Option<String> {
    let path = format!("/var/sessions/{}", session_id); // CWE-22: no sanitisation of session_id
    fs::read_to_string(path).ok()
}

// CWE-613: no session expiry — token valid forever
pub fn create_session(user_id: u64) -> String {
    format!("{}_{}", MASTER_TOKEN, user_id)           // CWE-613: static token, never expires
}

// CWE-319: credentials sent over HTTP
pub fn sync_credentials(username: &str, password: &str) {
    let url = format!(
        "http://auth-service.internal/sync?user={}&pass={}", // CWE-319: HTTP + CWE-598
        username, password
    );
    // reqwest::blocking::get(&url).ok();             // commented for compilation
    let _ = url;
}
