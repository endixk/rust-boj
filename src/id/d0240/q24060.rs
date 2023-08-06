// BOJ 24060 [Merge Sort 1]
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

fn go(a: &[u32], p: usize, q: usize, c: &mut usize, k: usize) {
    if p < q {
        let m = (p + q) >> 1;
        go(a, p, m, c, k);
        go(a, m+1, q, c, k);
        if k <= *c + q - p + 1 {
            let mut a = a[p..=q].to_vec();
            a.sort_unstable();
            println!("{}", a[k - *c - 1]);
            std::process::exit(0);
        } else {
            *c += q - p + 1;
        }
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<u32>(&mut it)).collect::<Vec<_>>();
    let mut c = 0;
    go(&a, 0, n-1, &mut c, k);
    println!("-1");
}
