// BOJ 15655 [N and M (6)]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn track(so: &mut io::BufWriter<io::StdoutLock>, n: usize, x: &[u16], v: &mut [u16], k: usize, bit: u32) {
    if k == 0 {
        v.iter().rev().for_each(|x| write!(so, "{} ", x).unwrap());
        writeln!(so).unwrap();
    } else {
        let mut mx = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                mx = i+1;
            }
        }
        for i in mx..n {
            if bit & (1 << i) == 0 {
                v[k - 1] = x[i];
                track(so, n, x, v, k - 1, bit | (1 << i));
            }
        }
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m) = (next(&mut it), next(&mut it));
    let mut x = it.map(|x| x.parse().unwrap()).collect::<Vec<u16>>();
    x.sort();
    track(&mut so, n, &x, &mut vec![0; m], m, 0);
}
