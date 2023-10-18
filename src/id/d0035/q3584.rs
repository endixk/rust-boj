// BOJ 3584 [Nearest Common Ancestor]
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

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut p = vec![0; n+1];
        for _ in 1..n {
            let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
            p[v] = u;
        }

        let (mut u, mut v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let (mut pu, mut pv) = (vec![u], vec![v]);
        while u > 0 { u = p[u]; pu.push(u); }
        while v > 0 { v = p[v]; pv.push(v); }

        let mut ans = 0;
        while !pu.is_empty() && !pv.is_empty() && pu.last().unwrap() == pv.last().unwrap() {
            ans = *pu.last().unwrap();
            pu.pop(); pv.pop();
        }
        writeln!(so, "{}", ans)?;
    }

    Ok(())
}
