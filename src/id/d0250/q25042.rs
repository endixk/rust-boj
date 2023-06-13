// BOJ 25042 [Dictation]
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

fn mm(x: u8, y: u8) -> u32 {
    match x {
        b'i' if y == b'j' || y == b'l' => 0,
        b'v' if y == b'w' => 0,
        _ if x == y => 0,
        _ => 1,
    }
}

fn edit(s: &[u8], t: &[u8], d: bool) -> u32 {
    let mut dp = (0..=t.len() as u32).collect::<Vec<_>>();
    for i in 0..s.len() {
        let mut tp = dp.clone();
        tp[0] = i as u32 + 1;
        for j in 1..=t.len() {
            tp[j] = dp[j-1] + if d { mm(s[i], t[j-1]) } else { mm(t[j-1], s[i]) };
            tp[j] = tp[j].min(dp[j] + 1).min(tp[j-1] + 1);
        }
        dp = tp;
    }
    dp[t.len()]
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (s1, s2) = (next::<String>(&mut it), next::<String>(&mut it));
    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
    if n > m {
        println!("{}", edit(s2, s1, false));
    } else {
        println!("{}", edit(s1, s2, true));
    }
}
