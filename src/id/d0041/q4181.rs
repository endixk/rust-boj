// BOJ 4181 [Convex Hull]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if t > 1e-9 { 1 } else if t < -1e-9 { -1 } else { 0 }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut p = (0..next(&mut it)).map(|_| {
        let x = next(&mut it);
        let y = next(&mut it);
        if next::<String>(&mut it) == "Y" {
            Some(Point { x, y })
        } else {
            None
        }
    }).filter_map(|x| x).collect::<Vec<_>>();

    let cent = p.iter().fold((0.0, 0.0), |(x, y), p| (x + p.x, y + p.y));
    let cent = Point { x: cent.0 / p.len() as f64, y: cent.1 / p.len() as f64 };

    p.sort_unstable_by(|a, b| {
        ccw(&cent, b, a).cmp(&0)
    });
    let p = p.iter().map(|p| (p.x as i32, p.y as i32)).collect::<Vec<_>>();
    let ori = p.iter().min().unwrap();
    let loc = p.iter().position(|p| p == ori).unwrap();

    writeln!(so, "{}", p.len()).ok();
    for i in 0..p.len() {
        let j = (i + loc) % p.len();
        writeln!(so, "{} {}", p[j].0, p[j].1).ok();
    }
}
