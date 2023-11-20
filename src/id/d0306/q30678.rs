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

const STAR: [[bool; 5]; 5] = [
    [false, false, true, false, false],
    [false, false, true, false, false],
    [true, true, true, true, true],
    [false, true, true, true, false],
    [false, true, false, true, false],
];

fn star(b: &mut Vec<Vec<bool>>, n: usize, c: usize, i: usize, j: usize) {
    if n == 0 { b[i][j] = true; return; }
    let (mut i, mut j) = (i, j);
    for x in 0..5 {
        for y in 0..5 {
            if STAR[x][y] {
                star(b, n-1, c/5, i, j);
            }
            j += c;
        }
        j -= 5*c;
        i += c;
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let c = 5usize.pow(n as u32);
    let mut b = vec![vec![false; c]; c];
    star(&mut b, n, c/5, 0, 0);
    for i in 0..c {
        for j in 0..c {
            write!(so, "{}", if b[i][j] { '*' } else { ' ' })?;
        }
        writeln!(so)?;
    }

    Ok(())
}