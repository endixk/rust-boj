// BOJ 25209 [ShakaShaka]
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

fn rect_vert(s: &[usize], x: &[usize], n: usize) -> bool {
    for i in 1..n {
        if s[i-1] != s[i] || x[i-1] != x[i] {
            return false;
        }
    }
    true
}
fn rect_diag(s: &[usize], x: &[usize], n: usize) -> bool {
    let mut p = 1;
    while p < n && s[p] == s[p-1] - 1 { p += 1; }
    let mut q = n-2;
    while q > 0 && s[q] == s[q+1] - 1 { q -= 1; }
    if p != q+1 { return false; }
    if s[p-1] != s[q+1] { return false; }

    p = 1;
    while p < n && s[p] + x[p] == s[p-1] + x[p-1] + 1 { p += 1; }
    q = n-2;
    while q > 0 && s[q] + x[q] == s[q+1] + x[q+1] + 1 { q -= 1; }
    if p != q+1 { return false; }
    if s[p-1] + x[p-1] != s[q+1] + x[q+1] { return false; }

    true
}

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
fn fill(b: &Vec<Vec<char>>, c: &mut Vec<Vec<i32>>, k: i32, i: usize, j: usize) {
    let mut q = std::collections::VecDeque::new();
    q.push_back((i, j)); c[i][j] = k;
    while let Some((i, j)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (i, j) = ((i as i32 + dx) as usize, (j as i32 + dy) as usize);
            if b[i][j] == '#' || c[i][j] != 0 { continue; }
            q.push_back((i, j)); c[i][j] = k;
        }
    }
}

fn scan(c: &Vec<Vec<i32>>, k: i32, n: usize, m: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    let mut s = Vec::new();
    let mut x = Vec::new();
    for i in 1..=n*3 {
        let mut flag = false;
        for j in 1..=m*3 {
            if c[i][j] == k { s.push(j); flag = true; break; }
        }
        if flag {
            let (mut j, mut l) = (*s.last().unwrap(), 0);
            while c[i][j] == k { j += 1; l += 1; }
            while j <= m*3 {
                if c[i][j] == k { return None; }
                j += 1;
            }
            x.push(l);
        }
    }
    Some((s, x))
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec!['#'; m*3+2]; n*3+2];
    for i in 1..=n*3 {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            b[i][j+1] = c;
        }
    }

    let mut c = vec![vec![0; m*3+2]; n*3+2];
    let mut k = 1;
    for i in 1..=n*3 {
        for j in 1..=m*3 {
            if b[i][j] == '#' || c[i][j] != 0 { continue; }
            fill(&b, &mut c, k, i, j);
            k += 1;
        }
    }

    for k in 1..k {
        if let Some((s, x)) = scan(&c, k, n, m) {
            if !rect_vert(&s, &x, s.len()) && !rect_diag(&s, &x, s.len()) {
                writeln!(so, "NO")?;
                return Ok(());
            }
        } else {
            writeln!(so, "NO")?;
            return Ok(());
        }
    }

    let mut t = vec![vec![5; m+2]; n+2];
    for i in 1..=n {
        for j in 1..=m {
            let mut sub = vec!['#'; 9];
            for x in 0..3 {
                for y in 0..3 {
                    sub[x*3+y] = b[i*3-2+x][j*3-2+y];
                }
            }
            match sub[4] {
                '0'..='4' => t[i][j] = sub[4] as i8 - b'0' as i8,
                _ => {
                    if sub.iter().filter(|&&c| c == '.').count() == 3 {
                        t[i][j] = -1;
                    }
                }
            }
        }
    }

    for i in 1..=n {
        for j in 1..=m {
            if t[i][j] == -1 { continue; }
            if t[i][j] == 5 { continue; }
            let cnt = if t[i-1][j] == -1 { 1 } else { 0 } +
                        if t[i+1][j] == -1 { 1 } else { 0 } +
                        if t[i][j-1] == -1 { 1 } else { 0 } +
                        if t[i][j+1] == -1 { 1 } else { 0 };
            if cnt != t[i][j] {
                writeln!(so, "NO")?;
                return Ok(());
            }
        }
    }

    writeln!(so, "YES")?;
    Ok(())
}
