#[inline]
pub fn gcd(mut a: u8, mut b: u8) -> u8 {
    if b > a {
        (a, b) = (b, a);
    }
    match (a, b) {
        (0, x) | (x, 0) => x,
        (1, _) | (_, 1) => 1,
        (a, b) => gcd(b, a % b),
    }
}

#[inline]
pub fn lcm(a: u8, b: u8) -> u8 {
    a * b / gcd(a, b)
}
