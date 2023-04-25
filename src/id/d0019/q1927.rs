// BOJ 1927 [Min Heap]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut heap = BinaryHeap::new();
    read(&mut si).split_ascii_whitespace().skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .for_each(|x| if x > 0 {
            heap.push(Reverse(x));
        } else {
            match heap.pop() {
                Some(Reverse(v)) => writeln!(so, "{}", v),
                None => writeln!(so, "0")
            }.unwrap();
        });
}
