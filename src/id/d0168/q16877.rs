// BOJ 16877 [Fimber]
// Supported by GitHub Copilot

use std::io::{self, Read};
fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}
fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

const MAX: usize = 3_000_001;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let fib = [
        1, 2, 3, 5, 8, 13, 21, 34, 55, 89,
        144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
        17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309
    ];

    let mut g = [0u8; MAX];
    for i in 1..MAX {
        let mut v = 0i32;
        for &k in fib.iter() {
            if k > i { break; }
            v |= 1 << g[i - k];
        }
        g[i] = v.trailing_ones() as u8;
    }

    let n = next::<usize>(&mut it);
    println!("{}",
             if (0..n).map(|_| next::<usize>(&mut it))
                 .fold(0, |acc, x| acc ^ g[x]) > 0
             { "koosaga" } else { "cubelover" });
}
