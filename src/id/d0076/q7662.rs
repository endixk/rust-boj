// BOJ 7662 [Dual Priority Queue]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    for _ in 0..it.by_ref().next().unwrap().parse().unwrap() {
        let mut min_heap = BinaryHeap::new();
        let mut max_heap = BinaryHeap::new();
        let mut map = HashMap::new();
        for _ in 0..it.by_ref().next().unwrap().parse().unwrap() {
            if it.by_ref().next().unwrap() == "I" {
                let n: i32 = it.by_ref().next().unwrap().parse().unwrap();
                min_heap.push(Reverse(n));
                max_heap.push(n);
                *map.entry(n).or_insert(0) += 1;
            } else {
                if it.by_ref().next().unwrap() == "1" {
                    while let Some(n) = max_heap.peek() {
                        if let Some(e) = map.get_mut(n) {
                            if *e == 0 {
                                max_heap.pop();
                            } else {
                                *e -= 1;
                                max_heap.pop();
                                break;
                            }
                        }
                    }
                } else {
                    while let Some(Reverse(n)) = min_heap.peek() {
                        if let Some(e) = map.get_mut(n) {
                            if *e == 0 {
                                min_heap.pop();
                            } else {
                                *e -= 1;
                                min_heap.pop();
                                break;
                            }
                        }
                    }
                }
            }
        }

        while let Some(n) = max_heap.peek() {
            if let Some(e) = map.get(n) {
                if *e == 0 {
                    max_heap.pop();
                } else { break; }
            }
        }
        while let Some(Reverse(n)) = min_heap.peek() {
            if let Some(e) = map.get(n) {
                if *e == 0 {
                    min_heap.pop();
                } else { break; }
            }
        }

        if !max_heap.is_empty(){
            writeln!(so, "{} {}", max_heap.peek().unwrap(), match min_heap.peek() {
                Some(Reverse(n)) => n,
                _ => &0,
            }).unwrap();
        } else {
            writeln!(so, "EMPTY").unwrap();
        }
    }
}
