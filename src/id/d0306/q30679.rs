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

fn go(b: &Vec<Vec<usize>>, i: usize, n: usize, m: usize) -> bool {
    let mut v = vec![vec![vec![false; 4]; m]; n];
    let mut dir = 0;
    let (mut i, mut j) = (i, 0);
    while !v[i][j][dir] {
        v[i][j][dir] = true;
        if dir == 0 { // right
            if j + b[i][j] >= m { return false; }
            j += b[i][j];
        } else if dir == 1 { // down
            if i + b[i][j] >= n { return false; }
            i += b[i][j];
        } else if dir == 2 { // left
            if j < b[i][j] { return false; }
            j -= b[i][j];
        } else {
            if i < b[i][j] { return false; }
            i -= b[i][j];
        }
        dir = (dir + 1) % 4;
    }
    true
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            b[i][j] = next::<usize>(&mut it);
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        if go(&b, i, n, m) {
            ans.push(i);
        }
    }

    writeln!(so, "{}", ans.len())?;
    ans.iter().for_each(|x| write!(so, "{} ", x+1).unwrap());
    writeln!(so)?;

    Ok(())
}