// BOJ 11003 [Minimum Values]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
fn read_int(si: &mut dyn Read) -> i64 {
    let mut buf = [0u8; 12];
    let mut i = 0;
    loop {
        si.read_exact(&mut buf[i..i+1]).unwrap();
        if buf[i] == b' ' || buf[i] == b'\n' { break; }
        i += 1;
    }
    std::str::from_utf8(&buf[..i]).unwrap().parse().unwrap()
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let (n, l) = (read_int(&mut si) as usize, read_int(&mut si) as usize);
    let mut q = std::collections::VecDeque::<(i32, usize)>::new();
    for i in 0..n {
        let x = read_int(&mut si) as i32;
        while !q.is_empty() && q.front().unwrap().1 + l <= i {
            q.pop_front();
        }
        while !q.is_empty() && q.back().unwrap().0 > x {
            q.pop_back();
        }
        q.push_back((x, i));
        write!(so, "{} ", q.front().unwrap().0)?;
    }
    writeln!(so)?;

    Ok(())
}
