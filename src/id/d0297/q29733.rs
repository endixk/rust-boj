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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (r, c, h) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![vec![false; c+2]; r+2]; h+2];
    for i in 1..=h {
        for j in 1..=r {
            let s = next::<String>(&mut it);
            for (k, c) in s.chars().enumerate() {
                b[i][j][k+1] = c == '*';
            }
        }
    }

    for i in 1..=h {
        for j in 1..=r {
            for k in 1..=c {
                if b[i][j][k] { write!(so, "*").ok(); }
                else {
                    let mut c = 0;
                    for p in i-1..i+2 {
                        for q in j-1..j+2 {
                            for r in k-1..k+2 {
                                if b[p][q][r] { c += 1; }
                            }
                        }
                    }
                    write!(so, "{}", c%10).ok();
                }
            }
            writeln!(so, "").ok();
        }
    }
}