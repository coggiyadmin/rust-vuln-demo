use std::process::Command;
use std::fs;
use std::io::{self, Read};

// CWE-798: hardcoded credentials
const DB_PASSWORD: &str = "prod_password_2024!";
const API_KEY: &str = "sk_live_rust_9xK2mNpQrT7vKwLzA4bC";
const JWT_SECRET: &str = "super-secret-jwt-signing-key";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        return;
    }

    match args[1].as_str() {
        "scan"   => scan_host(&args[2]),
        "read"   => read_file(&args[2]),
        "query"  => run_query(&args[2]),
        "exec"   => execute_command(&args[2]),
        _        => eprintln!("Unknown command"),
    }
}

// CWE-78: OS command injection — user input passed directly to shell
fn scan_host(host: &str) {
    // No sanitisation of host before constructing shell command
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("nmap -sV {}", host))             // CWE-78: injection via host
        .output()
        .expect("Failed to execute nmap");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

// CWE-22: path traversal — user input used directly in file path
fn read_file(path: &str) {
    // No canonicalisation or prefix check
    let contents = fs::read_to_string(path)            // CWE-22: path traversal
        .unwrap_or_else(|_| String::from("Error reading file"));
    println!("{}", contents);
}

// CWE-89: SQL injection via string formatting
fn run_query(username: &str) -> String {
    format!("SELECT * FROM users WHERE username = '{}'", username)  // CWE-89
}

// CWE-78: arbitrary command execution
fn execute_command(cmd: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)                                       // CWE-78: arbitrary shell execution
        .status()
        .expect("Command failed");
}
