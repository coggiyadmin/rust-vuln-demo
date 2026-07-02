use std::process::Command;
// ch03 gRPC payload → cmdi sink
pub fn grpc(payload: &str) {
    let _ = Command::new("sh").args(["-c", &format!("echo {}", payload)]); // SOURCE ch03
}
