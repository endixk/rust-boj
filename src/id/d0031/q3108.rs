// BOJ 3108 [LOGO]
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

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}

fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x == y {
        return;
    }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x1: i32, y1: i32, x2: i32, y2: i32
}
impl Rect {
    fn distinct(&self, other: &Rect) -> bool {
        // self is in other
        if self.x1 > other.x1 && self.y1 > other.y1 &&
           self.x2 < other.x2 && self.y2 < other.y2 {
            return true;
        }
        // other is in self
        if self.x1 < other.x1 && self.y1 < other.y1 &&
           self.x2 > other.x2 && self.y2 > other.y2 {
            return true;
        }
        // does not meet
        return self.x2 < other.x1 || self.x1 > other.x2 ||
               self.y2 < other.y1 || self.y1 > other.y2;
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = vec![Rect { x1: 0, y1: 0, x2: 0, y2: 0 }];
    for _ in 0..n {
        v.push(Rect {
            x1: next::<i32>(&mut it),
            y1: next::<i32>(&mut it),
            x2: next::<i32>(&mut it),
            y2: next::<i32>(&mut it)
        });
    }

    let mut root = (0..=n).collect::<Vec<usize>>();
    let mut rank = vec![0; n+1];
    for i in 0..n { for j in i+1..=n {
        if !v[i].distinct(&v[j]) {
            union(&mut root, &mut rank, i, j);
        }
    }}

    println!("{}", (0..=n).filter(|&i| root[i] == i).count() - 1);
}
