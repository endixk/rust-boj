// BOJ 10523 [Finding Lines]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

extern "C" {
    fn srand(seed: u64) -> u32;
    fn rand() -> u32;
}
fn rand_init() {
    unsafe {
        srand(31415926535);
    }
}
fn rand_int() -> u32 {
    unsafe {
        rand()
    }
}

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

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}

const RAND_CNT: usize = 88;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, p) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let points = (0..n).map(|_| Point { x: next::<i32>(&mut it), y: next::<i32>(&mut it) }).collect::<Vec<_>>();
    if n < 3 {
        println!("possible");
        return;
    }

    rand_init();
    for _ in 0..RAND_CNT {
        let x = rand_int() as usize % n;
        let y = rand_int() as usize % n;
        if x == y { continue; }

        let mut cnt = 2;
        for i in 0..n {
            if i == x || i == y { continue; }
            if ccw(&points[x], &points[y], &points[i]) == 0 {
                cnt += 1;
            }
        }

        if cnt >= (n * p + 99) / 100 {
            println!("possible");
            return;
        }
    }
    println!("impossible");

}
