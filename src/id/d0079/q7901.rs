// BOJ 7901 [Floors]
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

#[derive(Clone, Copy)] struct Point { x: i32, y: i32 }
#[derive(Clone, Copy)] struct Rect { p1: Point, p2: Point }
fn cut(v: &Vec<Rect>, w: i32, h: i32) -> Option<(i32, bool)> {
    let mut vmap = std::collections::HashMap::new();
    let mut hmap = std::collections::HashMap::new();
    for &r in v {
        if r.p2.x != w {
            let key = vmap.entry(r.p2.x).or_insert(0);
            *key += r.p2.y - r.p1.y;
            if *key == h {
                return Some((r.p2.x, true));
            }
        }
        if r.p2.y != h {
            let key = hmap.entry(r.p2.y).or_insert(0);
            *key += r.p2.x - r.p1.x;
            if *key == w {
                return Some((r.p2.y, false));
            }
        }
    }

    None
}
fn split(v: Vec<Rect>, k: i32, p: bool) -> (Vec<Rect>, Vec<Rect>) {
    let (mut v1, mut v2) = (vec![], vec![]);
    if p {
        for r in v {
            if r.p2.x <= k {
                v1.push(r);
            } else {
                v2.push(Rect {
                    p1: Point { x: r.p1.x - k, y: r.p1.y },
                    p2: Point { x: r.p2.x - k, y: r.p2.y }
                });
            }
        }
    } else {
        for r in v {
            if r.p2.y <= k {
                v1.push(r);
            } else {
                v2.push(Rect {
                    p1: Point { x: r.p1.x, y: r.p1.y - k },
                    p2: Point { x: r.p2.x, y: r.p2.y - k }
                });
            }
        }
    }
    (v1, v2)
}
fn go(r: Vec<Rect>, w: i32, h: i32) -> i32 {
    return if let Some((k, p)) = cut(&r, w, h) {
        let (v1, v2) = split(r, k, p);
        if p {
            go(v1, k, h).max(go(v2, w - k, h))
        } else {
            go(v1, w, k).max(go(v2, w, h - k))
        }
    } else {
        w * h
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let (w, h) = (next::<i32>(&mut it), next::<i32>(&mut it));
        let mut r = vec![];
        for _ in 0..next(&mut it) {
            let (x1, y1, x2, y2) = (
                next::<i32>(&mut it), next::<i32>(&mut it),
                next::<i32>(&mut it), next::<i32>(&mut it));
            r.push(Rect {
                p1: Point { x: x1, y: y1 },
                p2: Point { x: x2, y: y2 }
            });
        }
        writeln!(so, "{}", go(r, w, h))?;
    }

    Ok(())
}
