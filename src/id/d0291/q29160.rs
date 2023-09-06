// BOJ 29160 [FIFA Online]
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

fn go(pos: &Vec<Vec<usize>>, x: usize, p: usize, k: usize) -> bool {
    let mut c = 0;
    for &v in &pos[p] {
        if v < x { break }
        c += v - x + 1;
        if c >= k { return true }
    }
    false
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it) + 1);
    let mut pos = vec![vec![]; 11];
    for _ in 0..n {
        let (p, w) = (next::<usize>(&mut it), next::<usize>(&mut it));
        pos[p-1].push(w);
    }
    for pv in pos.iter_mut() {
        pv.sort_unstable_by(|a, b| b.cmp(a));
    }

    let mut ans = 0;
    for p in 0..11 {
        if pos[p].is_empty() { continue }
        let (mut l, mut r) = (0, 100000);
        while l <= r {
            let m = (l + r) >> 1;
            if go(&pos, m, p, k) { l = m + 1 }
            else { r = m - 1 }
        }
        ans += r;
    }
    println!("{}", ans);
}
