// BOJ 2502 [Tiger and Rice Cakes]
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

    let mut f = vec![0; 28];
    f[0] = 1; f[1] = 1;
    for i in 2..28 { f[i] = f[i-1] + f[i-2]; }

    let (d, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    for i in 1..k {
        if (k - i * f[d-3]) % f[d-2] == 0 {
            writeln!(so, "{}\n{}", i, (k - i * f[d-3]) / f[d-2])?;
            break;
        }
    }

    Ok(())
}
