// BOJ 28476 [Butterflies 2]
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
    x: i64,
    y: i64,
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let points = (0..n).map(|_| { Point { x: next(&mut it), y: next(&mut it) } }).collect::<Vec<_>>();

    let mut k = 0;
    for i in 0..n {
        let mut p = points.iter().enumerate()
            .filter(|&(j, p)| i != j && p.x <= points[i].x)
            .map(|(_, p)| p).collect::<Vec<_>>();
        let mut q = points.iter().filter(|&p| p.x > points[i].x).collect::<Vec<_>>();
        p.sort_unstable_by(|a, b| ccw(&points[i], a, b).cmp(&0));
        q.sort_unstable_by(|a, b| ccw(&points[i], a, b).cmp(&0));
        p.extend(q);
        p.extend(p.clone());

        let mut c = 0;
        for j in 0..n-1 {
            if c == j { c += 1; }
            while ccw(&points[i], &p[j], &p[c]) < 0 {
                c += 1;
            }
            let x = c-j-1;
            if x > 1 {
                k += x*(x-1)/2;
            }
        }
    }
    println!("{}", 2*k-n*(n-1)*(n-2)*(n-3)/4);
}
