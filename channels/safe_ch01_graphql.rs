// TN — GraphQL id numeric gate.
pub fn safe_graphql(id: &str) -> bool {
    id.chars().all(|c| c.is_ascii_digit())
}
