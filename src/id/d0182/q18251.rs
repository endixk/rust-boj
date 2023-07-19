// BOJ 18251 [Simple DFS Problem]
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

fn inorder(v: &mut Vec<i64>, a: &Vec<i64>, i: usize, n: usize) {
    let (l, r) = (i<<1, (i<<1)+1);
    if l <= n { inorder(v, a, l, n); }
    v.push(a[i]);
    if r <= n { inorder(v, a, r, n); }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = vec![0; n+1];
    (1..=n).for_each(|i| a[i] = next::<i64>(&mut it));

    let mut v = Vec::new();
    inorder(&mut v, &a, 1, n);

    let mut ans = -1_000_000_007;
    for h1 in 0..(n+1).trailing_zeros() as usize {
        for h2 in h1..(n+1).trailing_zeros() as usize {
            let (mut max, mut sum) = (ans, 0);
            for i in 1..=n {
                let z = i.trailing_zeros() as usize;
                if z < h1 || z > h2 { continue; }
                sum += v[i-1];
                max = max.max(sum);
                if sum < 0 { sum = 0; }
            }
            ans = ans.max(max);
        }
    }
    println!("{}", ans);
}
