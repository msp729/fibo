// recursive soln
// 42 and 43 straddle one second
fn recur(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => recur(n - 1) + recur(n - 2),
    }
}

// 74
fn grec(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        10 => 55,
        20 => 6765,
        _ => grec(n - 3) + 2 * grec(n - 2),
    }
}
