// BOJ 11723 [Set]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut bit = 0;
    for _ in 0..read(&mut si).trim().parse().unwrap() {
        let s = read(&mut si);
        let mut it = s.split_ascii_whitespace();
        let cmd = it.by_ref().next().unwrap();
        let n: u8 = match cmd {
            "all" | "empty" => 0,
            _ => it.next().unwrap().parse().unwrap()
        };
        match cmd {
            "add" => bit |= 1 << n,
            "remove" => bit &= !(1 << n),
            "check" => writeln!(so, "{}", match bit & 1 << n { 0 => 0, _ => 1}).unwrap(),
            "toggle" => bit ^= 1 << n,
            "all" => bit |= 2097151,
            "empty" => bit = 0,
            _ => ()
        }
    }
}
