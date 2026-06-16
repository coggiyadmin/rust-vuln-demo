//! BENIGN-BASELINE TRUE-NEGATIVE FIXTURE.
//!
//! Ordinary business logic with NO security surface: no HTTP, no DB, no file
//! I/O, no exec, no crypto, no secrets. The scanner MUST produce ZERO security
//! findings here. Measures specificity / the noise floor.

use std::collections::HashMap;

/// A product line with a unit price and quantity.
pub struct Item {
    pub sku: String,
    pub unit_price: f64,
    pub quantity: u32,
}

impl Item {
    fn extended(&self) -> f64 {
        self.unit_price * self.quantity as f64
    }
}

fn to_cents(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

/// Sum of all line extended prices, rounded to cents.
pub fn subtotal(items: &[Item]) -> f64 {
    to_cents(items.iter().map(|i| i.extended()).sum())
}

/// Pick a discount tier from a subtotal.
pub fn tier_for(amount: f64) -> &'static str {
    if amount >= 1000.0 {
        "gold"
    } else if amount >= 250.0 {
        "silver"
    } else {
        "standard"
    }
}

/// Final total after the tier discount.
pub fn total(items: &[Item]) -> f64 {
    let sub = subtotal(items);
    let rate = match tier_for(sub) {
        "gold" => 0.10,
        "silver" => 0.05,
        _ => 0.0,
    };
    to_cents(sub - sub * rate)
}

/// Group items by the category code before the dash in the SKU.
pub fn by_category(items: Vec<Item>) -> HashMap<String, Vec<Item>> {
    let mut groups: HashMap<String, Vec<Item>> = HashMap::new();
    for it in items {
        let code = match it.sku.find('-') {
            Some(i) if i > 0 => it.sku[..i].to_string(),
            _ => "misc".to_string(),
        };
        groups.entry(code).or_default().push(it);
    }
    groups
}
