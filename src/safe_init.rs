// NEGATIVE TEST FILE — safe mirror of init_harvest.rs.
//
// The scanner MUST produce ZERO security findings here. Any finding is a
// FALSE POSITIVE.

use std::thread;
use std::time::Duration;

pub fn register_safe_init() {
    thread::spawn(|| {
        // SAFE init — no credential reads, no network exfiltration.
        thread::sleep(Duration::from_millis(100));
    });
}
