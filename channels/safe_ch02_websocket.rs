// TN — WebSocket HTML escape.
pub fn safe_websocket(msg: &str) -> String {
    msg.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
