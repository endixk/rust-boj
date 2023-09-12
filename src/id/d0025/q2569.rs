// BOJ 2569 [Organizing]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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

fn cycle(a: &[(usize, usize)], i: usize, n: usize) -> Vec<bool> {
    let mut ret = vec![false; n];
    let mut j = i;
    while !ret[j] {
        ret[j] = true;
        j = a[j].1;
    }
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = Vec::new();
    for i in 0..n {
        a.push((next::<usize>(&mut it), i));
    }
    a.sort_unstable();

    let mut ans = 0;
    let mut vis = vec![false; n];
    for i in 0..n {
        if !vis[i] {
            let c = cycle(&a, i, n);
            let (mut cmin, mut csum, mut csz, mut emin) = (99999, 0, 0, 99999);
            for j in 0..n {
                if c[j] {
                    csum += a[j].0;
                    cmin = cmin.min(a[j].0);
                    csz += 1;
                    vis[j] = true;
                } else {
                    emin = emin.min(a[j].0);
                }
            }
            ans += ((csz - 1) * cmin + csum - cmin).min((csz + 1) * emin + csum + cmin);
        }
    }
    writeln!(so, "{}", ans).unwrap();
}
