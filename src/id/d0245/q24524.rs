// BOJ 24524 [Beautiful String]
// Supported by GitHub Copilot

use std::io::{self, Read};

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
    let (s, t) = (next::<String>(&mut it), next::<String>(&mut it));

    let mut map = [0; 26];
    for (i, &c) in t.as_bytes().into_iter().enumerate() {
        map[c as usize - 97] = i+1;
    }

    let mut prog = [0; 26];
    for &c in s.as_bytes().into_iter() {
        match map[c as usize - 97] {
            0 => continue,
            1 => prog[0] += 1,
            p if prog[p-2] > 0 => { prog[p-1] += 1; prog[p-2] -= 1; },
            _ => continue,
        }
    }

    println!("{}", prog[t.len()-1]);
}
