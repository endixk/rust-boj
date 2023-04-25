// BOJ 2798 [Black Jack]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    si.read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    buf.clear();

    si.read_line(&mut buf).unwrap();
    let v: Vec<i32> = buf.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut ans = 0;
    for i in 0..n {
        for j in i+1..n {
            for k in j+1..n {
                let sum = v[i] + v[j] + v[k];
                if sum <= m && sum > ans {
                    ans = sum;
                }
            }
        }
    }

    writeln!(so, "{}", ans).unwrap();
}
