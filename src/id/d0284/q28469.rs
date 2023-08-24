// BOJ 28469 [Equilibrium Points]
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

fn force(x: &[f64], q: &[f64], n: usize, loc: f64) -> f64 {
    let mut ret = q[0] / (loc - x[0]).powi(2);
    for i in 1..n {
        ret -= q[i] / (x[i] - loc).powi(2);
    }
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = vec![(0, 0); n];
    for i in 0..n {
        a[i].0 = next::<i32>(&mut it);
    }
    for i in 0..n {
        a[i].1 = next::<i32>(&mut it);
    }
    a.sort_unstable();
    let x = a.iter().map(|&(x, _)| x as f64).collect::<Vec<_>>();
    let q = a.iter().map(|&(_, q)| q as f64).collect::<Vec<_>>();

    let (mut lo, mut hi) = (x[0], x[1]);
    loop {
        if hi - lo < 1e-6 { break; }
        let mi = (lo + hi) / 2.0;
        if force(&x, &q, n, mi) > 0.0 { lo = mi; }
        else { hi = mi; }
    }
    println!("{:.6}", lo);
}
