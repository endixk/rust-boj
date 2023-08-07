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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            v[i][j] = next::<i64>(&mut it);
        }
    }

    let mut a = Vec::new();
    for i in 0..n { a.push(v[i][0]); }
    for j in 0..m { a.push(v[0][0] - v[0][j]); }
    a.sort_unstable();
    let (mut p, mut c, mut x, mut k) = (a[0], 1, a[0], 1);
    for q in a.into_iter().skip(1) {
        if p != q {
            if c > k { x = p; k = c; }
            p = q; c = 1;
        } else {
            c += 1;
        }
    }
    if c > k { x = p; }

    let mut ans = Vec::new();
    let mut r = vec![0; n];
    let mut c = vec![0; m];
    for i in 0..n {
        if v[i][0] != x {
            r[i] = v[i][0] - x;
            ans.push((1, i+1, r[i]));
        }
    }
    for j in 0..m {
        if v[0][0] - v[0][j] != x {
            c[j] = x - v[0][0] + v[0][j];
            ans.push((2, j+1, c[j]));
        }
    }

    for i in 0..n {
        for j in 0..m {
            if r[i] + c[j] != v[i][j] {
                writeln!(so, "-1").ok();
                return;
            }
        }
    }

    writeln!(so, "{}", ans.len()).ok();
    for (x, i, v) in ans {
        writeln!(so, "{} {} {}", x, i, v).ok();
    }
}
