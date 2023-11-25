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

#[derive(Eq, PartialEq, Clone, Copy, Debug)] struct Point { x: i32, y: i32, m: bool }
fn dsq(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = (0..n).map(|_| Point { x: next(&mut it), y: next(&mut it), m: true }).collect::<Vec<_>>();
    let p = (0..m).map(|_| next::<usize>(&mut it) - 1).collect::<Vec<_>>();
    let r0 = next::<i64>(&mut it);
    let mut r = vec![0; n];
    for i in 0..m {
        let x = next::<i64>(&mut it);
        r[p[i]] = r[p[i]].max(x);
    }

    for i in p {
        for j in 0..n {
            if dsq(&v[i], &v[j]) <= r[i].pow(2) {
                v[j].m = false;
            }
        }
    }

    let mut x = vec![false; n];
    let mut q = std::collections::VecDeque::new();
    for i in 0..n {
        if v[i].m { x[i] = true; q.push_back(i); }
    }
    while let Some(i) = q.pop_front() {
        for j in 0..n {
            if x[j] { continue; }
            if dsq(&v[i], &v[j]) <= r0.pow(2) {
                x[j] = true;
                q.push_back(j);
            }
        }
    }

    writeln!(so, "{}", x.iter().filter(|&&b| b).count())?;

    Ok(())
}