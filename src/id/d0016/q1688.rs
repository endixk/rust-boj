// BOJ 1688 [Terror]
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
fn within(a: i32, b: i32, x: i32) -> bool {
    return if a < b { a <= x && x <= b } else { b <= x && x <= a }
}
fn inside(poly: &Vec<Point>, p: &Point) -> bool {
    let mut cnt = 0;
    for i in 0..poly.len() {
        let j = (i+1) % poly.len();
        let (x, y) = if poly[i].y < poly[j].y { (&poly[i], &poly[j]) } else { (&poly[j], &poly[i]) };

        if ccw(x, y, p) == 0 && within(x.x, y.x, p.x) && within(x.y, y.y, p.y) {
            return true; // p is on the line
        }
        if x.y <= p.y && p.y < y.y && ccw(x, y, p) > 0 {
            cnt += 1;
        }
    }
    cnt & 1 == 1
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let poly = (0..n).map(|_| Point { x: next(&mut it), y: next(&mut it) }).collect::<Vec<_>>();
    for _ in 0..3 {
        let p = Point { x: next(&mut it), y: next(&mut it) };
        println!("{}", if inside(&poly, &p) { 1 } else { 0 });
    }
}
