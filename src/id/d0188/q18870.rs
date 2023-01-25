// BOJ 18870 [Coordinate Compression]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut v: Vec<(i32, usize)> = read(&mut si).split_ascii_whitespace().skip(1)
        .map(|x| x.parse::<i32>().unwrap()).zip(0..).collect();
    v.sort();

    let mut ans = vec![0; v.len()];
    let mut ord = 0;
    let mut pre = v[0].0;
    for x in v.iter().skip(1) {
        if x.0 != pre {
            ord += 1;
            pre = x.0;
        }
        ans[x.1] = ord;
    }

    ans.iter().for_each(|x| write!(so, "{} ", x).unwrap());
}
