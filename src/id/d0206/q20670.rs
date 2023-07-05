// BOJ 20670 [Mystery Sign]
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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn inside(poly: &Vec<Point>, p: &Point, n: usize) -> bool {
    if ccw(&poly[0], &poly[1], p) < 0 || ccw(&poly[0], &poly[n-1], p) > 0 {
        return false;
    }
    let (mut l, mut r) = (1, n-1);
    while l+1 < r {
        let m = (l+r) / 2;
        if ccw(&poly[0], &poly[m], p) > 0 { l = m; }
        else { r = m; }
    }
    ccw(&poly[l], &poly[r], p) > 0
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| Point { x: next(&mut it), y: next(&mut it) }).collect::<Vec<_>>();
    let b = (0..m).map(|_| Point { x: next(&mut it), y: next(&mut it) }).collect::<Vec<_>>();
    let c = (0..k).map(|_| Point { x: next(&mut it), y: next(&mut it) }).collect::<Vec<_>>();

    let mut ans = k;
    for &p in &c {
        if inside(&a, &p, n) && !inside(&b, &p, m) { ans -= 1; }
    }
    println!("{}", if ans == 0 { "YES".to_string() } else { ans.to_string() })
}
