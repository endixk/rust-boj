// BOJ 28238 [Ambition]
// Supported by GitHub Copilot

use std::io::{self, Read};

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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| {
        (0..5).map(|_| next::<u8>(&mut it) == 1).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let (mut ans, mut max) = ((0, 1), 0);
    for i in 0..4 { for j in i+1..5 {
        let cnt = v.iter().filter(|&x| x[i] && x[j]).count();
        if cnt > max {
            max = cnt;
            ans = (i, j);
        }
    }}

    println!("{}", max);
    (0..5).for_each(|i| {
        if ans.0 == i || ans.1 == i {
            print!("{} ", 1);
        } else {
            print!("{} ", 0);
        }
    });
}

