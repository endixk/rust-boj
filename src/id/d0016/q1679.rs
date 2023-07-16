// BOJ 1679 [Number Game]
// Supported by GitHub Copilot

use std::io::{self, Read};
use std::collections::VecDeque;

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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let k = next::<usize>(&mut it);

    const MAX: usize = 50000;
    let mut vis = vec![false; MAX+2];
    vis[0] = true;

    let mut q = VecDeque::new();
    q.push_back(0);
    let mut r = 0;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let x = q.pop_front().unwrap();
            for &y in &v {
                if x + y > MAX || vis[x+y] { continue; }
                vis[x+y] = true;
                q.push_back(x+y);
            }
        }
        r += 1;
        if r == k { break; }
    }

    let m = vis.iter().position(|&x| !x).unwrap();
    match m & 1 {
        0 => println!("holsoon win at {}", m),
        _ => println!("jjaksoon win at {}", m),
    }
}
