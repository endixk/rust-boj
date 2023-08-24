// BOJ 28466 [Best Team]
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

fn map(fate: &[u8]) -> (usize, usize) {
    ((fate[0] - b'0') as usize, (fate[1] - b'A') as usize)
}
fn update(v: &mut [[u8; 3]; 120], x: u8, fate: &[u8]) {
    let f = map(fate);
    let f= f.0 * 12 + f.1;
    if x > v[f][2] { v[f][2] = x; }
    if v[f][2] > v[f][1] { v[f].swap(1, 2); }
    if v[f][1] > v[f][0] { v[f].swap(0, 1); }
}
fn score(f: &[(usize, usize); 3], v: &[[u8; 3]; 120], a: &[[u8; 10]; 10], b: &[[u8; 12]; 12]) -> u8 {
    let mut ret = 0;
    let mut ptr = [0; 120];

    for i in 0..3 {
        let f = f[i].0 * 12 + f[i].1;
        if v[f][ptr[f]] == 0 { return 0; }
        ret += v[f][ptr[f]];
        ptr[f] += 1;
    }

    ret + a[f[0].0][f[1].0]
        + a[f[0].0][f[2].0]
        + a[f[1].0][f[2].0]
        + b[f[0].1][f[1].1]
        + b[f[0].1][f[2].1]
        + b[f[1].1][f[2].1]
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = [[0; 10]; 10];
    let mut b = [[0; 12]; 12];
    for i in 0..10 { for j in 0..10 {
        a[i][j] = next::<u8>(&mut it);
    }}
    for i in 0..12 { for j in 0..12 {
        b[i][j] = next::<u8>(&mut it);
    }}

    let mut v = [[0; 3]; 120];
    for _ in 0..n {
        let (x, fate) = (next::<u8>(&mut it), next::<String>(&mut it));
        update(&mut v, x, fate.as_bytes());
    }

    let mut ans = 0;
    for i in 0..120 {
        if v[i][0] == 0 { continue; }
        let fi = (i / 12, i % 12);
        for j in 0..120 {
            if v[j][0] == 0 { continue; }
            let fj = (j / 12, j % 12);
            for k in 0..120 {
                if v[k][0] == 0 { continue; }
                let fk = (k / 12, k % 12);
                ans = ans.max(score(&[fi, fj, fk], &v, &a, &b));
            }
        }
    }
    println!("{}", ans);
}
