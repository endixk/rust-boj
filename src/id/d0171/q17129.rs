// BOJ 17129 [Williamson's Sapsuckers]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![1; 9020000];
    let mut x = 0;
    for i in 1..=n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            let c = c as u8 - b'0';
            let c = if c > 2 { 3 } else { c };
            b[i*(m+2)+j+1] = c;
            if c == 2 { x = i*(m+2)+j+1; }
        }
    }

    let mut q = std::collections::VecDeque::new();
    let mut v = vec![false; 9020000];
    let mut d = 0;
    v[x] = true; q.push_back(x);
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let x = q.pop_front().unwrap();
            for y in [x-m-2, x+m+2, x-1, x+1] {
                if b[y] == 1 { continue; }
                if v[y] { continue; }
                if b[y] == 3 {
                    writeln!(so, "TAK\n{}", d+1)?;
                    return Ok(());
                }
                v[y] = true; q.push_back(y);
            }
        }
        d += 1;
    }
    writeln!(so, "NIE")?;

    Ok(())
}
