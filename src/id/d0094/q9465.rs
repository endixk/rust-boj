// BOJ 9465 [Stickers]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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
        let n: usize = it.by_ref().next().unwrap().parse().unwrap();
        let vx: Vec<i32> = it.by_ref().take(n).map(|x| x.parse().unwrap()).collect();
        let vy: Vec<i32> = it.by_ref().take(n).map(|x| x.parse().unwrap()).collect();

        let mut d = (0, vx[0], vy[0]);
        for i in 1..n {
            d = (
                d.1.max(d.2).max(d.0),
                d.0.max(d.2) + vx[i],
                d.0.max(d.1) + vy[i],
                );
        }

        writeln!(so, "{}", d.0.max(d.1).max(d.2)).unwrap();
    }
}
