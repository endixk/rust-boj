// BOJ 7568 [Body Sizes]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let v = (0..next(&mut it)).map(|_|
        (next::<usize>(&mut it), next::<usize>(&mut it))
    ).collect::<Vec<_>>();

    let mut ans = vec![1; v.len()];
    for i in 0..v.len() {
        for j in 0..v.len() {
            if v[i].0 < v[j].0 && v[i].1 < v[j].1 {
                ans[i] += 1;
            }
        }
    }

    ans.iter().for_each(|x| write!(so, "{} ", x).unwrap());
    writeln!(so).ok();
}
