// BOJ 5525 [IOIOI]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let s = next::<String>(&mut it);
    let s = s.as_bytes();

    let mut v = Vec::new();
    let mut i = 0;
    while i < m {
        if i + 1 == m { break }
        while s[i] != 'I' as u8 || s[i+1] != 'O' as u8 { i += 1; if i + 1 == m { break } }
        if i + 1 == m { break }

        let mut p = 1;
        while i+3 < m && s[i+2] == 'I' as u8 && s[i+3] == 'O' as u8 {
            i += 2; p += 1;
        }
        if i + 2 == m { v.push(p-1); }
        else { v.push(p - if s[i+2] == 'I' as u8 { 0 } else { 1 })}
        i += 2;
    }

    let mut ans = 0;
    v.iter().for_each(|&x| if x >= n { ans += x - n + 1 });
    writeln!(so, "{}", ans).unwrap();
}
