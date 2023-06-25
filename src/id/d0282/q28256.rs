// BOJ 28256 [Chocolate Storage]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};

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

    for _ in 0..next(&mut it) {
        let mut c = vec![];
        next::<String>(&mut it).chars().for_each(|x| c.push(x == 'O'));
        let s = next::<String>(&mut it);
        let s = s.as_bytes();
        c.push(s[2] == b'O');
        let k = s[0] == b'O';
        next::<String>(&mut it).chars().rev().for_each(|x| c.push(x == 'O'));
        c.push(k);

        let v = (0..next(&mut it)).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
        if (0..8).all(|x| c[x]) {
            if v.len() == 1 && v[0] == 8 {
                writeln!(so, "1").ok();
            } else {
                writeln!(so, "0").ok();
            }
            continue;
        }

        let x = c.iter().enumerate().find(|&(_, &x)| !x).unwrap().0;
        let mut w = vec![];
        let (mut n, mut t) = (0, (x+1)%8);
        while t != x {
            if c[t] { n += 1; }
            else {
                if n > 0 { w.push(n); }
                n = 0;
            }
            t = (t+1)%8;
        }
        if n > 0 { w.push(n); }
        w.sort_unstable();
        if v == w {
            writeln!(so, "1").ok();
        } else {
            writeln!(so, "0").ok();
        }
    }
}
