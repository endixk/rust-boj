// BOJ 2146 [Build a Bridge]
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

const DX: [i32; 4] = [0, 0, -1, 1];
const DY: [i32; 4] = [-1, 1, 0, 0];
fn go(b: &Vec<Vec<bool>>, c: &mut Vec<Vec<bool>>, i: usize, j: usize, n: usize) -> usize {
    // flood fill
    let mut v = vec![vec![false; n+2]; n+2];
    let mut f = Vec::new();
    let mut q = std::collections::VecDeque::new();
    q.push_back((i, j));
    f.push((i, j));
    v[i][j] = true; c[i][j] = true;
    while let Some((i, j)) = q.pop_front() {
        let (i, j) = (i as i32, j as i32);
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (i, j) = ((i+dx) as usize, (j+dy) as usize);
            if b[i][j] && !v[i][j] {
                q.push_back((i, j));
                f.push((i, j));
                v[i][j] = true; c[i][j] = true;
            }
        }
    }

    for (i, j) in f {
        q.push_back((i, j));
    }
    let mut d = 1;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let (i, j) = q.pop_front().unwrap();
            let (i, j) = (i as i32, j as i32);
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                if i + dx < 1 || i + dx > n as i32 || j + dy < 1 || j + dy > n as i32 { continue; }
                let (i, j) = ((i+dx) as usize, (j+dy) as usize);
                if !v[i][j] {
                    if b[i][j] {
                        return d;
                    }
                    q.push_back((i, j));
                    v[i][j] = true;
                }
            }
        }
        d += 1;
    }
    d
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut b = vec![vec![false; n+2]; n+2];
    for i in 1..=n { for j in 1..=n {
        b[i][j] = next::<u8>(&mut it) == 1;
    }}

    let mut c = vec![vec![false; n+2]; n+2];
    let mut ans = n*n;
    for i in 1..=n { for j in 1..=n {
        if b[i][j] && !c[i][j] {
            ans = ans.min(go(&b, &mut c, i, j, n));
        }
    }}

    writeln!(so, "{}", ans-1)?;

    Ok(())
}
