// BOJ 30416 [Let's Move]
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

#[derive(Eq, PartialEq, Clone, Copy, Debug)] struct Point {
    x: i32,
    y: i32,
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn inside_convex(poly: &Vec<Point>, p: &Point, n: usize) -> bool {
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
#[inline] fn sort(poly: &mut Vec<Point>) {
    let ori = poly[0];
    poly[1..].sort_by(|a, b| {
        0.cmp(&ccw(&ori, a, b))
    });
}
#[inline] fn within(a: i32, b: i32, x: i32) -> bool {
    return if a < b { a <= x && x <= b } else { b <= x && x <= a }
}
#[inline] fn on(a: &Point, b: &Point, c: &Point) -> bool {
    ccw(a, b, c) == 0 && within(a.x, b.x, c.x) && within(a.y, b.y, c.y)
}
fn on_polygon(poly: &Vec<Point>, p: &Point, n: usize) -> bool {
    for i in 0..n {
        let j = (i+1) % n;
        if on(&poly[i], &poly[j], p) { return true; }
    }
    false
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut sq = vec![];
    for _ in 0..n {
        let mut poly = vec![
            Point { x: next::<i32>(&mut it), y: next::<i32>(&mut it) },
            Point { x: next::<i32>(&mut it), y: next::<i32>(&mut it) },
            Point { x: next::<i32>(&mut it), y: next::<i32>(&mut it) },
            Point { x: next::<i32>(&mut it), y: next::<i32>(&mut it) },
        ];
        sort(&mut poly);
        sq.push(poly);
    }

    let mut ans = 0;
    for i in 0..n {
        let mut c = 0;
        for j in 0..n {
            if  (on_polygon(&sq[i], &sq[j][0], 4) || inside_convex(&sq[i], &sq[j][0], 4)) &&
                (on_polygon(&sq[i], &sq[j][1], 4) || inside_convex(&sq[i], &sq[j][1], 4)) &&
                (on_polygon(&sq[i], &sq[j][2], 4) || inside_convex(&sq[i], &sq[j][2], 4)) &&
                (on_polygon(&sq[i], &sq[j][3], 4) || inside_convex(&sq[i], &sq[j][3], 4)) {
                c += 1;
            }
        }
        ans = ans.max(c);
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
