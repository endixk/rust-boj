// BOJ 7562 [Knight Moves]
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

const DX: [i32; 8] = [-2, -1, 1, 2, 2, 1, -1, -2];
const DY: [i32; 8] = [1, 2, 2, 1, -1, -2, -2, -1];
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let n = next::<usize>(&mut it);
        let mut v = vec![vec![-1; n]; n];
        let (sx, sy) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let (ex, ey) = (next::<usize>(&mut it), next::<usize>(&mut it));

        let mut q = std::collections::VecDeque::new();
        v[sx][sy] = 0; q.push_back((sx+1, sy+1));
        while let Some((x, y)) = q.pop_front() {
            if x == ex+1 && y == ey+1 {
                writeln!(so, "{}", v[x-1][y-1])?;
                break;
            }
            let (i, j) = (x as i32, y as i32);
            for (&di, &dj) in DX.iter().zip(DY.iter()) {
                if i + di < 1 || i + di > n as i32 || j + dj < 1 || j + dj > n as i32 { continue; }
                let (i, j) = ((i + di) as usize, (j + dj) as usize);
                if v[i-1][j-1] != -1 { continue; }
                v[i-1][j-1] = v[x-1][y-1] + 1;
                q.push_back((i, j));
            }
        }
    }

    Ok(())
}
