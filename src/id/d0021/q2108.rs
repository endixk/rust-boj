// BOJ 2108 [Statistics]
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
    let n = next::<usize>(&mut it);

    let v = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut c = vec![0; 8001];
    let mut sum = 0;
    v.iter().for_each(|&x| {
        sum += x;
        c[(x + 4000) as usize] += 1;
    });

    let cmax = c.iter().max().unwrap();
    let mut mc = Vec::new();
    let (mut mid, mut acc) = (None, 0);
    for (i, &k) in c.iter().enumerate() {
        acc += k;
        if acc > n/2 && mid.is_none() {
            mid = Some(i as i32 - 4000);
        }
        if k == *cmax {
            mc.push(i as i32 - 4000);
        }
    }

    writeln!(so, "{}", (sum as f64 / n as f64).round() as i32).unwrap();
    writeln!(so, "{}", mid.unwrap()).unwrap();
    writeln!(so, "{}", if mc.len() > 1 { mc[1] } else { mc[0] }).unwrap();
    writeln!(so, "{}", c.iter().enumerate().filter(|(_, &k)| k > 0).last().unwrap().0
        - c.iter().enumerate().filter(|(_, &k)| k > 0).next().unwrap().0).unwrap();
}
