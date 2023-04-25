// BOJ 10826 [Fibonacci Number 4]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::iter::repeat;
fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn big_sum(x: &[u8], y: &[u8]) -> Vec<u8> {
    let mut carry = 0;
    let mut z = y.iter().zip(x.iter().chain(repeat(&0)))
        .map(|(a, b)| {
            let c = a + b + carry;
            carry = c / 10;
            c % 10
        })
        .collect::<Vec<u8>>();
    if carry > 0 {
        z.push(carry);
    }
    z
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let n: usize = read(&mut si).trim().parse().unwrap();
    let mut fx = vec![0];
    let mut fy = vec![1];
    for _ in 0..n {
        let fz = big_sum(&fx, &fy);
        fx = fy;
        fy = fz;
    }

    fx.iter().rev().for_each(|&a| write!(so, "{}", a).unwrap());
}
