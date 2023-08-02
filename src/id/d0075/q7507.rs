// BOJ 7507 [Olympic Games]
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

    for tc in 1..=next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut v = (0..n).map(|_| {
            let (d, s, e) = (
                next::<i32>(&mut it),
                next::<i32>(&mut it),
                next::<i32>(&mut it)
            );
            d<<24 | e<<12 | s
        }
        ).collect::<Vec<_>>();
        v.sort_unstable();

        let (mut cnt, mut lst) = (0, 0);
        for x in v {
            let (d, e, s) = (x>>24, x>>12&0xfff, x&0xfff);
            if d<<12|s >= lst {
                cnt += 1;
                lst = d<<12|e;
            }
        }

        writeln!(so, "Scenario #{}:", tc).unwrap();
        writeln!(so, "{}", cnt).unwrap();
        writeln!(so).unwrap();
    }
}
