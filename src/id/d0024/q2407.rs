// BOJ 2407 [Combination]
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

    let v: Vec<u128> = read(&mut si).split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut c: u128 = 1;

    let n = v[0];
    let r = if v[1] > n/2 { n-v[1] } else { v[1] };
    let mut v: Vec<bool> = vec![false; r as usize + 1];
    for x in (n-r+1 ..= n).rev() {
        c *= x;
        for i in 2 ..= r {
            if c % i == 0 && v[i as usize] == false {
                c /= i;
                v[i as usize] = true;
            }
        }
    }

    writeln!(so, "{}", c).unwrap();
}
