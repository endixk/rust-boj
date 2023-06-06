// BOJ 20529 [MBTI]
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

fn hash(mbti: String) -> u8 {
    let mut ret = 0;
    for c in mbti.chars() {
        ret |= match c {
            'E' => 1,
            'S' => 2,
            'T' => 4,
            'J' => 8,
            _ => 0,
        };
    }
    ret
}
fn dist(h1: u8, h2: u8) -> u8 {
    let mut x = h1 ^ h2;
    let mut ret = 0;
    while x > 0 {
        ret += x & 1;
        x >>= 1;
    }
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut a = vec![0; 16];
        for _ in 0..n {
            let mbti = next::<String>(&mut it);
            a[hash(mbti) as usize] += 1;
        }
        if a.iter().filter(|&x| *x > 2).count() > 0 { println!("0"); continue; }

        let mut v = vec![];
        for i in 0..16 {
            for _ in 0..a[i] {
                v.push(i as u8);
            }
        }

        let mut ans = 12;
        for i in 0..n-2 { for j in i+1..n-1 { for k in j+1..n{
            let d = dist(v[i], v[j]) + dist(v[j], v[k]) + dist(v[i], v[k]);
            if d < ans { ans = d; }
        }}}
        println!("{}", ans);
    }
}
