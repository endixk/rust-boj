// BOJ 25196 [Birds]
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

fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, x, y) = xgcd(b, a % b);
    (g, y, x - (a / b) * y)
}
fn find(m: i64, n: i64, x: i64, y: i64) -> Option<i64> {
    let (g, k, _) = xgcd(m, n);
    if (y - x) % g != 0 { return None; }
    let k = k * (y - x) / g;
    let g = m * n / g;
    let mut k = (m * k + x) % g;
    while k < 0 { k += g; }
    Some(k)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (xv, xs, xe) = (next::<i64>(&mut it), next::<i64>(&mut it), next::<i64>(&mut it));
    let (yv, ys, ye) = (next::<i64>(&mut it), next::<i64>(&mut it), next::<i64>(&mut it));
    let (zv, zs, ze) = (next::<i64>(&mut it), next::<i64>(&mut it), next::<i64>(&mut it));

    let mut ans = i64::MAX;
    for i in 0..xv*yv {
        if i % xv < xs || i % xv > xe { continue; }
        if i % yv < ys || i % yv > ye { continue; }
        if let Some(k) = find(xv*yv, zv, i, zs) {
            ans = ans.min(k);
        }
    }
    for i in 0..xv*zv {
        if i % xv < xs || i % xv > xe { continue; }
        if i % zv < zs || i % zv > ze { continue; }
        if let Some(k) = find(xv*zv, yv, i, ys) {
            ans = ans.min(k);
        }
    }
    for i in 0..yv*zv {
        if i % yv < ys || i % yv > ye { continue; }
        if i % zv < zs || i % zv > ze { continue; }
        if let Some(k) = find(yv*zv, xv, i, xs) {
            ans = ans.min(k);
        }
    }
    writeln!(so, "{}", if ans == i64::MAX { -1 } else { ans }).unwrap();
}
