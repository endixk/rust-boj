// BOJ 1270 [Territory War]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn readline<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    (0..next(&mut readline(&mut si).split_ascii_whitespace())).for_each(|_| {
        let s = readline(&mut si);
        let mut it = s.split_ascii_whitespace();
        let v = (0..next(&mut it)).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

        let (mut m, mut c) = (0, 0);
        v.iter().for_each(|&x| {
            if m == x { c += 1; }
            else if c == 0 { m = x; c = 1; }
            else { c -= 1; }
        });
        c = 0;
        v.iter().for_each(|&x| if x == m { c += 1; });
        writeln!(so, "{}", if (c<<1) > v.len() as i64 { m.to_string() } else { "SYJKGW".to_string() }).ok();
    })
}
