// BOJ 17144 [Farewell to Fine Dust]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

const DX: [i8; 4] = [0, 1, 0, -1];
const DY: [i8; 4] = [1, 0, -1, 0];
fn diffuse(a: &mut Vec<Vec<i32>>, r: usize, c: usize) {
    let mut d = vec![vec![0; c]; r];
    for x in 0..r { for y in 0..c {
        let k = a[x][y] / 5;
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (nx, ny) = (x as i8 + dx, y as i8 + dy);
            if nx < 0 || nx >= r as i8 || ny < 0 || ny >= c as i8 { continue; }

            let (nx, ny) = (nx as usize, ny as usize);
            if a[nx][ny] == -1 { continue; }

            d[nx][ny] += k;
            d[x][y] -= k;
        }
    }}
    for x in 0..r { for y in 0..c { a[x][y] += d[x][y]; }}
}

fn circulate(a: &mut Vec<Vec<i32>>, k: usize, r: usize, c: usize, top: bool) {
    if top {
        for x in (1..k).rev() { a[x][0] = a[x-1][0]; } // sweep up
        for y in 0..c-1 { a[0][y] = a[0][y+1]; } // sweep right
        for x in 0..k { a[x][c-1] = a[x+1][c-1]; } // sweep down
        for y in (1..c).rev() { a[k][y] = a[k][y-1]; } // sweep left
    } else {
        for x in k+1..r-1 { a[x][0] = a[x+1][0]; } // sweep down
        for y in 0..c-1 { a[r-1][y] = a[r-1][y+1]; } // sweep right
        for x in (k+1..r).rev() { a[x][c-1] = a[x-1][c-1]; } // sweep up
        for y in (1..c).rev() { a[k][y] = a[k][y-1]; } // sweep left
    }
    a[k][1] = 0;
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (r, c, t) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![0; c]; r];
    let (mut k1, mut k2) = (0, 0);
    for x in 0..r { for y in 0..c {
        a[x][y] = next::<i32>(&mut it);
        if a[x][y] == -1 {
            if k1 == 0 { k1 = x; }
            else { k2 = x; }
        }
    }}

    (0..t).for_each(|_| {
        diffuse(&mut a, r, c);
        circulate(&mut a, k1, r, c, true);
        circulate(&mut a, k2, r, c, false);
    });

    writeln!(so, "{}", a.iter().flatten().filter(|&&x| x > 0).sum::<i32>()).unwrap();
}
