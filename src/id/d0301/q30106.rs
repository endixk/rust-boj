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

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
fn fill(b: &Vec<Vec<i64>>, v: &mut Vec<Vec<bool>>, k: i64, i: usize, j: usize, n: usize, m: usize) {
    let mut q = std::collections::VecDeque::new();
    q.push_back((i, j)); v[i][j] = true;
    while let Some((i, j)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (x, y) = (i as i32 + dx, j as i32 + dy);
            if x < 0 || x >= n as i32 || y < 0 || y >= m as i32 { continue; }
            let (x, y) = (x as usize, y as usize);
            if v[x][y] || (b[x][y] - b[i][j]).abs() > k { continue; }
            q.push_back((x, y)); v[x][y] = true;
        }
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it));
    let mut b = vec![vec![0; m]; n];
    for i in 0..n { for j in 0..m {
        b[i][j] = next(&mut it);
    }}

    let mut ans = 0;
    let mut v = vec![vec![false; m]; n];
    for i in 0..n { for j in 0..m {
        if v[i][j] { continue; }
        fill(&b, &mut v, k, i, j, n, m);
        ans += 1;
    }}

    writeln!(so, "{}", ans)?;

    Ok(())
}