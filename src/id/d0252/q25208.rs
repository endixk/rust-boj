// BOJ 25208 [Detective at Dawn]
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

const MAP: [usize; 32] = [
    3, 0, 2, 5, 4, 1, 0, 0,
    4, 1, 0, 3, 5, 2, 0, 0,
    1, 5, 2, 0, 4, 3, 0, 0,
    2, 1, 5, 3, 0, 4, 0, 0
];
const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
#[inline] fn go(z: usize, d: usize) -> usize {
    let (s, x, y) = (z>>18, z>>9&511, z&511);
    let (x, y) = ((x as i32+DX[d]) as usize, (y as i32+DY[d]) as usize);
    return MAP[d<<3|s]<<18|x<<9|y;
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, _) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut w = [false; 262626];
    let mut v = [false; 1599999];
    let (mut dx, mut rx) = (0, 0);
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            match c {
                '#' => w[i<<9|j] = true,
                'D' => { dx = i<<9|j }
                'R' => {
                    rx = i<<9|j;
                    for s in 1..6 { v[s<<18|rx] = true; }
                },
                _ => (),
            }
        }
    }

    let mut q = std::collections::VecDeque::new();
    q.push_back(dx); v[dx] = true;
    let mut k = 0;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let z = q.pop_front().unwrap();
            for d in 0..4 {
                let nz = go(z, d);
                if w[nz&262143] || v[nz] { continue; }
                if nz == rx {
                    writeln!(so, "{}", k+1)?;
                    return Ok(());
                }
                v[nz] = true;
                q.push_back(nz);
            }
        }
        k += 1;
    }
    writeln!(so, "-1")?;

    Ok(())
}
