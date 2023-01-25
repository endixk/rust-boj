// BOJ 11286 [Absolute Min Heap]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

#[derive(Eq, PartialEq)]
struct Abs {
    v: i32
}

impl Abs {
    fn new(v: i32) -> Self {
        Abs { v }
    }
}

impl PartialOrd for Abs {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Abs {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.v.abs() == other.v.abs() {
            true => self.v.cmp(&other.v),
            false => self.v.abs().cmp(&other.v.abs())
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut heap = BinaryHeap::new();
    read(&mut si).split_ascii_whitespace().skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .for_each(|x| if x != 0 {
            heap.push(Reverse(Abs::new(x)));
        } else {
            match heap.pop() {
                Some(Reverse(v)) => writeln!(so, "{}", v.v),
                None => writeln!(so, "0")
            }.unwrap();
        });
}
