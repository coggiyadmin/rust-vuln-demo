// TN — benign template fill; validated typed fields, no AI surface.
#![allow(dead_code)]

pub fn render(user: &str, amount: i64) -> Result<String, String> {
    if amount < 0 {
        return Err("amount must be non-negative".into());
    }
    Ok(format!("Hi {}, your balance changed by {} credits.", user, amount))
}
