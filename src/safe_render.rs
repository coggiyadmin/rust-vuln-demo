//! SAFE / TN fixture — HTML rendering with contextual escaping.
//! Untrusted strings are HTML-escaped before insertion into markup. The scanner
//! MUST produce ZERO security findings; any xss finding is a FALSE POSITIVE.

/// Escape the five HTML-significant characters.
pub fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Render a profile card; both fields are escaped before interpolation.
pub fn render_profile(display_name: &str, bio: &str) -> String {
    format!(
        "<div class=\"card\"><h2>{}</h2><p>{}</p></div>",
        escape_html(display_name),
        escape_html(bio),
    )
}
