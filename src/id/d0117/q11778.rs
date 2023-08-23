// BOJ 11778 [Fibonacci Numbers and GCD]
// Supported by GitHub Copilot

struct Mat {
    x00: i64,
    x01: i64,
    x10: i64,
    x11: i64,
}
fn mat_mul(a: &Mat, b: &Mat, m: i64) -> Mat {
    Mat {
        x00: (a.x00 * b.x00 + a.x01 * b.x10) % m,
        x01: (a.x00 * b.x01 + a.x01 * b.x11) % m,
        x10: (a.x10 * b.x00 + a.x11 * b.x10) % m,
        x11: (a.x10 * b.x01 + a.x11 * b.x11) % m,
    }
}
fn pow(a: &Mat, p: i64, m: i64) -> Mat {
    if p == 1 {
        return Mat{x00: a.x00, x01: a.x01, x10: a.x10, x11: a.x11};
    }
    let b = pow(a, p / 2, m);
    let c = mat_mul(&b, &b, m);
    if p % 2 == 0 {
        c
    } else {
        mat_mul(&c, a, m)
    }
}
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
pub fn main() {
    let v = std::io::stdin().lines().next().unwrap().unwrap()
        .split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    println!("{}", pow(&Mat{x00: 1, x01: 1, x10: 1, x11: 0}, gcd(v[0], v[1]), 1_000_000_007).x10);
}
