// BOJ 28018 [Does the Schedule Overlaps?]
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

    const SZ: usize = 1000003;
    let n = next::<usize>(&mut it);
    let mut arr = vec![0i32; SZ];
    (0..n).for_each(|_| {
        arr[next::<usize>(&mut it)] += 1;
        arr[next::<usize>(&mut it) + 1] -= 1;
    });
    let mut pre = vec![0i32; SZ];
    for i in 1..SZ {
        pre[i] = pre[i-1] + arr[i];
    }

    (0..next(&mut it)).for_each(|_| {
        let q = next::<usize>(&mut it);
        writeln!(so, "{}", pre[q]).ok();
    })
}

