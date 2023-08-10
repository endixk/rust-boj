// BOJ 28252 [Leaf Sequence]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};

fn read_int(si: &mut dyn Read) -> usize {
    let mut buf = [0u8; 10];
    let mut i = 0;
    loop {
        si.read_exact(&mut buf[i..i+1]).unwrap();
        if buf[i] == b' ' || buf[i] == b'\n' { break; }
        i += 1;
    }
    std::str::from_utf8(&buf[..i]).unwrap().parse().unwrap()
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::with_capacity(1<<21, io::stdout().lock());

    let n = read_int(&mut si);
    let mut k = 0;
    let mut x = read_int(&mut si);
    k += x;

    for _ in 1..n {
        let y = read_int(&mut si);
        if y > x { println!("-1"); std::process::exit(0); }
        if x + y == 2 { println!("-1"); std::process::exit(0); }
        for j in 0..y { writeln!(so, "{} {}", k-x+j+1, k+j+1).ok(); }
        for j in y..x { writeln!(so, "{} {}", k-x+j+1, k+y).ok(); }
        k += y;
        x = y;
    }
    if x == 2 { writeln!(so, "{} {}", k-1, k).ok(); }
    else if x > 2 { println!("-1"); std::process::exit(0); }
    println!("{k}");
}
