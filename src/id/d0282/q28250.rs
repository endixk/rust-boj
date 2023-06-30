// BOJ 28250 [MEX]
// Supported by GitHub Copilot

use std::io::{self, Read};
pub fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let (mut z, mut o, mut e) = (0i64, 0i64, 0i64);
    s.split_ascii_whitespace().skip(1).for_each(|s| {
        match s.parse().unwrap() {
            0 => z += 1,
            1 => o += 1,
            _ => e += 1,
        }
    });
    println!("{}", z*(z-1)/2 + o*z*2 + e*z);
}
