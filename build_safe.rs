// NEGATIVE TEST FILE — safe mirror of build.rs (reference build script).
//
// Production projects would use this pattern instead of exfiltrating credentials.
// The scanner MUST produce ZERO security findings here.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
}
