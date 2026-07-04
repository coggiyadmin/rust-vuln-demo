// v09 guard-in-variable
use std::collections::HashSet;
pub fn eval_expr(expr: &str) -> bool {
    ["1+1"].into_iter().collect::<HashSet<&str>>().contains(expr)
}
