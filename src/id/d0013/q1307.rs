// BOJ 1307 [Magic Square]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

fn gen_odd(n: usize) -> Vec<Vec<u32>> {
    let mut v = vec![vec![0; n]; n];
    let (mut x, mut y) = (0, n / 2);
    for i in 1..=n * n {
        v[x][y] = i as u32;
        let (nx, ny) = ((x + n - 1) % n, (y + 1) % n);
        if v[nx][ny] != 0 {
            x = (x + 1) % n;
        } else {
            x = nx;
            y = ny;
        }
    }
    v
}

fn gen_dev(n: usize) -> Vec<Vec<u32>> {
    let mut v = vec![vec![0; n]; n];
    let mut f = vec![vec![false; n]; n];
    let pat = [
        [true, false, false, true],
        [false, true, true, false],
        [false, true, true, false],
        [true, false, false, true],
    ];

    for i in 0..n {
        for j in 0..n {
            f[i][j] = pat[i % 4][j % 4];
        }
    }

    let mut x = 1;
    for i in 0..n { for j in 0..n {
        if f[i][j] { v[i][j] = x; }
        x += 1;
    }}
    x = 1;
    for i in (0..n).rev() { for j in (0..n).rev() {
        if !f[i][j] { v[i][j] = x; }
        x += 1;
    }}
    v
}

// credit: https://destiny738.tistory.com/246
fn gen_sev(n: usize) -> Vec<Vec<u32>> {
    let k = n / 4;
    let mut tl = vec![vec![0; n/2]; n/2];
    for i in 0..n/2 { for j in 0..k {
        tl[i][j] = 3;
    }}
    tl[k][0] = 0;
    tl[k][k] = 3;
    let bl = tl.iter().map(|v| v.iter().map(|&x| 3 - x).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut tr = vec![vec![2; n/2]; n/2];
    for i in 0..n/2 { for j in 1..k {
        tr[i][n/2-j] = 1;
    }}
    let br = tr.iter().map(|v| v.iter().map(|&x| 3 - x).collect::<Vec<_>>()).collect::<Vec<_>>();

    let sq = gen_odd(n / 2);
    let mut v = vec![vec![0; n]; n];
    for i in 0..n/2 { for j in 0..n/2 {
        v[i][j] = sq[i][j] + tl[i][j] * (n * n / 4) as u32;
        v[i][j+n/2] = sq[i][j] + tr[i][j] * (n * n / 4) as u32;
        v[i+n/2][j] = sq[i][j] + bl[i][j] * (n * n / 4) as u32;
        v[i+n/2][j+n/2] = sq[i][j] + br[i][j] * (n * n / 4) as u32;
    }}
    v
}

pub fn main() {
    let n = io::stdin().lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    let sq = if n % 2 == 1 { gen_odd(n) } else if n % 4 == 0 { gen_dev(n) } else { gen_sev(n) };

    let mut so = io::BufWriter::new(io::stdout().lock());
    for i in 0..sq.len() {
        for j in 0..sq.len() {
            write!(so, "{} ", sq[i][j]).unwrap();
        }
        writeln!(so).unwrap();
    }
}
