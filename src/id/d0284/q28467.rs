// BOJ 28467 [Spell Cards]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = vec![0; n+2];
    for i in 1..=n {
        a[i] = next::<usize>(&mut it);
    }
    a[0] = i32::MAX as usize;
    a[n+1] = i32::MAX as usize;

    let mut v = a.iter().enumerate().skip(1).take(n).map(|(i, &x)| (x, i)).collect::<Vec<_>>();
    v.sort_unstable();
    let mut f = vec![false; n+2];
    let mut ans = 0;
    for (_, i) in v.into_iter().take(n-1) {
        f[i] = true;
        let (mut l, mut r) = (i-1, i+1);
        while f[l] { l -= 1; }
        while f[r] { r += 1; }
        ans += a[i] + a[l].min(a[r]);
    }
    println!("{}", ans);
}
