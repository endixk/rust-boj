// BOJ 1920 [Find a Number]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut buf = String::new();
    let n = {
        si.read_line(&mut buf).unwrap();
        buf.trim().parse::<usize>().unwrap()
    };
    buf.clear();

    let mut a = vec![0; n];
    si.read_line(&mut buf).unwrap();
    for (i, x) in buf.trim().split_whitespace().enumerate() {
        a[i] = x.parse::<i32>().unwrap();
    }
    buf.clear();
    a.sort();

    si.read_line(&mut String::new()).unwrap();
    si.read_line(&mut buf).unwrap();
    for x in buf.trim().split_whitespace() {
        let x = x.parse::<i32>().unwrap();

        // Binary search
        let mut l = 0;
        let mut r = n;
        while l < r {
            let m = (l + r) / 2;
            if a[m] < x {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if l < n && a[l] == x {
            writeln!(so, "1").unwrap();
        } else {
            writeln!(so, "0").unwrap();
        }
    }
}
