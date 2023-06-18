// BOJ 13269 [Building Blocks]
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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut top = vec![vec![false; m]; n];
    for i in 0..n { for j in 0..m {
        top[i][j] = next::<u8>(&mut it) == 1;
    }}
    let front = (0..m).map(|_| next::<u8>(&mut it)).collect::<Vec<_>>();
    let mut side = (0..n).map(|_| next::<u8>(&mut it)).collect::<Vec<_>>();
    side.reverse();

    let mut ans = vec![vec![0; m]; n];
    for i in 0..n { for j in 0..m {
        if top[i][j] {
            ans[i][j] = side[i].min(front[j]);
        }
    }}

    for i in 0..n {
        let sh = ans[i].iter().max().unwrap();
        if *sh != side[i] {
            writeln!(so, "-1").unwrap();
            return;
        }
    }
    for j in 0..m {
        let sh = (0..n).map(|i| ans[i][j]).max().unwrap();
        if sh != front[j] {
            writeln!(so, "-1").unwrap();
            return;
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(so, "{} ", ans[i][j]).unwrap();
        }
        writeln!(so).unwrap();
    }
}
