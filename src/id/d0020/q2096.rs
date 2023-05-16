// BOJ 2096 [Descent]
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

    let n: usize = next(&mut it);
    let mut dmax = [next::<u32>(&mut it), next::<u32>(&mut it), next::<u32>(&mut it)];
    let mut dmin = dmax.clone();
    for _ in 1..n {
        let a = [next::<u32>(&mut it), next::<u32>(&mut it), next::<u32>(&mut it)];

        let t = dmax.clone();
        dmax[0] = a[0] + t[0].max(t[1]);
        dmax[1] = a[1] + t[0].max(t[1]).max(t[2]);
        dmax[2] = a[2] + t[1].max(t[2]);

        let t = dmin.clone();
        dmin[0] = a[0] + t[0].min(t[1]);
        dmin[1] = a[1] + t[0].min(t[1]).min(t[2]);
        dmin[2] = a[2] + t[1].min(t[2]);
    }
    writeln!(so, "{} {}",
             dmax[0].max(dmax[1]).max(dmax[2]),
             dmin[0].min(dmin[1]).min(dmin[2])
    ).ok();
}
