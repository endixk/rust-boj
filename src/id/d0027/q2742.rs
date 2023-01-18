// BOJ 2742 [N tnirP]
// Supported by GitHub Copilot

use std::io;
use std::fmt::Write;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut buf = String::new();
    for i in (1..=n).rev() {
        writeln!(buf, "{}", i).unwrap();
    }
    print!("{}", buf);
}
