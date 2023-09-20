// BOJ 25682 [Chessboard Painting 2]
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

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut board = vec![false; 4000000];
    let mut p = vec![false; 4000000];
    let mut flag = false;
    for i in 0..n {
        let s = next::<String>(&mut it);
        let mut f = flag;
        for (j, c) in s.chars().enumerate() {
            board[i*2000+j] = c == 'B';
            p[i*2000+j] = c == if f { 'B' } else { 'W' };
            f = !f;
        }
        flag = !flag;
    }

    let mut ps = vec![0; 4004001];
    let mut ans = (n*m) as i32;
    let ksq = (k*k) as i32;
    for i in 0..n { for j in 0..m {
        let (x, y) = (i*2000, k*2000);
        ps[x+2000+j+1] = ps[x+j+1] + ps[x+2000+j] - ps[x+j] + p[x+j] as i32;
        if i >= k-1 && j >= k-1 {
            let x = ps[x+2000+j+1] - ps[x+2000-y+j+1] - ps[x+2000+j+1-k] + ps[x+2000-y+j+1-k];
            ans = ans.min(x);
            ans = ans.min(ksq - x);
        }
    }}
    writeln!(so, "{}", ans)?;

    Ok(())
}
