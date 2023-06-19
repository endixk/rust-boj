// BOJ 28240 [S-League]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let (k, l, c, j) = (
        next::<usize>(&mut it)-1, next::<usize>(&mut it)-1,
        next::<usize>(&mut it)-1, next::<usize>(&mut it)-1,
    );

    let (mut fc, mut fj) = (false, false);
    let mut i = k;
    while i != l {
        i = (i + 1) % n;
        if i == c {
            fc = true;
        } else if i == j {
            fj = true;
        }
    }

    let mut ans = vec![(0, 0); n];
    i = k;
    if fc ^ fj {
        ans[k] = (-99999999, 0);
        ans[l] = (0, 0);
        let mut y = 0;
        i = l;
        loop {
            i = (i + 1) % n;
            if i == k { break; }
            y -= 1;
            ans[i] = (0, y);
        }
        y = 0;
        i = l;
        loop {
            i = if i == 0 { n - 1 } else { i - 1 };
            if i == k { break; }
            y += 1;
            ans[i] = (0, y);
        }
        ans[j] = (1, ans[j].1);
        ans[c] = (1, ans[c].1);
    } else {
        let (mut x, mut y) = (0, 0);
        if fc {
            while i != c && i != j {
                i = (i + 1) % n;
                y += 1;
                ans[i] = (x, y);
            }
            x += 1;
            loop {
                i = (i + 1) % n;
                if i == k { break; }
                y -= 1;
                ans[i] = (x, y);
            }
        } else {
            i = l;
            while i != c && i != j {
                i = (i + 1) % n;
                y += 1;
                ans[i] = (x, y);
            }
            x += 1;
            loop {
                i = (i + 1) % n;
                if i == l { break; }
                y -= 1;
                ans[i] = (x, y);
            }
        }
    }

    for i in 0..n {
        writeln!(so, "{} {}", ans[i].0, ans[i].1).ok();
    }
}

