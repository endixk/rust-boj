// BOJ 28015 [Coloring Areas]
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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![0; m+1]; n+1];
    for i in 1..=n {
        for j in 1..=m {
            a[i][j] = next::<usize>(&mut it);
        }
    }

    let mut ans = 0;
    for i in 1..=n {
        let mut j = 1;
        loop {
            while j <= m && a[i][j] == 0 { j += 1; }
            if j > m { break; }

            let (mut x, mut y) = (0, 0);
            while j <= m {
                if a[i][j] == 0 { break; }
                if a[i][j] == 1 && a[i][j-1] != 1 { x += 1; }
                if a[i][j] == 2 && a[i][j-1] != 2 { y += 1; }
                j += 1;
            }

            if x == 0 { ans += y; }
            else if y == 0 { ans += x; }
            else { ans += x.min(y) + 1; }
        }
    }
    writeln!(so, "{}", ans).ok();
}

