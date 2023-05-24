// BOJ 9935 [EKSPLOZIJA]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let t = read(&mut si).chars().collect::<Vec<char>>();
    let n = t.len();

    let mut vc = Vec::<char>::new();
    let mut vi = Vec::<u8>::new();
    vc.push(' '); vi.push(0);
    for c in s.chars() {
        let &i = vi.last().unwrap();
        vc.push(c);
        vi.push(if c == t[i as usize] { i+1 } else if c == t[0] { 1 } else { 0 });
        if let Some(&i) = vi.last() {
            if i == n as u8 {
                for _ in 0..n { vc.pop(); vi.pop(); }
            }
        }
    }

    if vc.len() == 1 {
        println!("FRULA");
    } else {
        println!("{}", vc.iter().skip(1).collect::<String>());
    }
}
