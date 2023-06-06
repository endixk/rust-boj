// BOJ 4354 [Power Strings]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn pi(s: &String) -> Vec<usize> {
    let (s, l) = (s.as_bytes(), s.len());
    let mut p = vec![0; l];

    let mut j = 0;
    for i in 1..l {
        while j > 0 && s[i] != s[j] { j = p[j-1]; }
        if s[i] == s[j] { j += 1; p[i] = j; }
    }

    p
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    loop {
        let s = read(&mut si);
        if s == "." { break; }
        let p = pi(&s);
        let (n, m) = (s.len(), s.len() - p[s.len()-1]);
        println!("{}", if n % m == 0 { n / m } else { 1 });
    }
}
