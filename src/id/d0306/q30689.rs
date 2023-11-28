// BOJ 30689 [Maze Repair]
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

fn cycle(c: &mut Vec<Vec<i32>>, b: &Vec<Vec<char>>, x: i32, i: usize, j: usize) {
    if c[i][j] != 0 { return; }
    c[i][j] = x;
    match b[i][j] {
        'U' => cycle(c, b, x, i - 1, j),
        'D' => cycle(c, b, x, i + 1, j),
        'L' => cycle(c, b, x, i, j - 1),
        'R' => cycle(c, b, x, i, j + 1),
        _ => (),
    }
}
fn find(v: &mut Vec<Vec<i32>>, x: i32, y: &mut i32, c: &mut Vec<Vec<i32>>, b: &Vec<Vec<char>>,
        i: usize, j: usize, n: usize, m: usize) {
    if v[i][j] == x { cycle(c, b, *y, i, j); *y += 1; return; }
    if v[i][j] != 0 { return; }
    v[i][j] = x;
    match b[i][j] {
        'U' => if i > 0   { find(v, x, y, c, b, i - 1, j, n, m); },
        'D' => if i < n-1 { find(v, x, y, c, b, i + 1, j, n, m); },
        'L' => if j > 0   { find(v, x, y, c, b, i, j - 1, n, m); },
        'R' => if j < m-1 { find(v, x, y, c, b, i, j + 1, n, m); },
        _ => (),
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![' '; m]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            b[i][j] = c;
        }
    }

    let mut c = vec![vec![0; m]; n];
    let mut v = vec![vec![0; m]; n];
    let (mut x, mut y) = (1, 1);
    for i in 0..n { for j in 0..m {
        if v[i][j] == 0 {
            find(&mut v, x, &mut y, &mut c, &b, i, j, n, m);
            x += 1;
        }
    }}

    let mut a = vec![-1; y as usize];
    for i in 0..n { for j in 0..m {
        let d = next::<i32>(&mut it);
        let c = c[i][j] as usize;
        if a[c] == -1 || a[c] > d { a[c] = d; }
    }}

    writeln!(so, "{}", a[1..].iter().sum::<i32>())?;

    Ok(())
}
