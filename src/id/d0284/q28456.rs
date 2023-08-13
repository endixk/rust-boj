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

fn ra(a: Vec<Vec<u16>>, x: usize, n: usize) -> Vec<Vec<u16>> {
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        if i == x {
            ret[i][0] = a[i][n-1];
            for j in 1..n {
                ret[i][j] = a[i][j-1];
            }
        } else {
            ret[i] = a[i].clone();
        }
    }
    ret
}
fn rb(a: Vec<Vec<u16>>, n: usize) -> Vec<Vec<u16>> {
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            ret[j][n-i-1] = a[i][j];
        }
    }
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            a[i][j] = next::<u16>(&mut it);
        }
    }

    for _ in 0..next(&mut it) {
        match next::<u8>(&mut it) {
            1 => a = ra(a, next::<usize>(&mut it)-1, n),
            _ => a = rb(a, n),
        }
    }
    for i in 0..n {
        for j in 0..n {
            write!(so, "{} ", a[i][j]).unwrap();
        }
        writeln!(so).unwrap();
    }
}