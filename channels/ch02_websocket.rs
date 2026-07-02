// ch02 WebSocket message → XSS sink
pub fn websocket(msg: &str) {
    let _ = format!("<p>{}</p>", msg); // SOURCE ch02
}
