// BOJ 2162 [Group of Line Segments]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::cmp::{min, max};

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

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct Point { x: i64, y: i64 }
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if t > 0 { 1 }
    else if t < 0 { -1 }
    else { 0 }
}

struct Line { a: Point, b: Point }
fn intersect(l1: &Line, l2: &Line) -> bool {
    let ab = ccw(&l1.a, &l1.b, &l2.a) * ccw(&l1.a, &l1.b, &l2.b);
    let cd = ccw(&l2.a, &l2.b, &l1.a) * ccw(&l2.a, &l2.b, &l1.b);
    if ab == 0 && cd == 0 {
        let p1 = min(&l1.a, &l1.b);
        let p2 = max(&l1.a, &l1.b);
        let p3 = min(&l2.a, &l2.b);
        let p4 = max(&l2.a, &l2.b);
        return if p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4 { true }
        else { p2 >= p3 && p4 >= p1 }
    }
    ab <= 0 && cd <= 0
}

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}

fn union_size(root: &mut Vec<usize>,
              rank: &mut Vec<usize>,
              size: &mut Vec<usize>, x: usize, y: usize) -> usize {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            size[y] += size[x];
            size[y]
        } else {
            root[y] = x;
            size[x] += size[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
            size[x]
        }
    } else {
        size[x]
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let lines = (0..n).map(|_| {
        let x1 = next::<i64>(&mut it);
        let y1 = next::<i64>(&mut it);
        let x2 = next::<i64>(&mut it);
        let y2 = next::<i64>(&mut it);
        Line { a: Point { x: x1, y: y1 }, b: Point { x: x2, y: y2 } }
    }).collect::<Vec<_>>();

    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut size = vec![1; n];
    for i in 0..n-1 { for j in i+1..n {
        if intersect(&lines[i], &lines[j]) {
            union_size(&mut root, &mut rank, &mut size, i, j);
        }
    }}

    let (mut c, mut m) = (0, 0);
    for i in 0..n {
        if root[i] == i {
            c += 1;
            m = max(m, size[i]);
        }
    }
    writeln!(so, "{} {}", c, m).unwrap();
}
