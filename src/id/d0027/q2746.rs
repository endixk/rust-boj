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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    a.sort_unstable();

    use std::collections::HashMap;
    let (mut l, mut r) = (HashMap::new(), HashMap::new());
    let mut x = a[0]; l.insert(a[0], 0);
    for i in 1..n {
        if a[i] != x {
            r.insert(x, i - 1);
            x = a[i];
            l.insert(x, i);
        }
    }
    r.insert(x, n - 1);

    let mut p = vec![0; n];
    p[0] = a[0];
    for i in 1..n {
        p[i] = p[i - 1] + a[i];
    }

    let mut ans = 0;
    for i in 0..n-1 {
        let x = p[n-2] - a[i] - a[n-1];
        if x < a[i] { continue; }
        else if x == a[i] {
            ans += if x == a[n-1] { n-2 } else { r[&x] } - i;
        }
        else if r.contains_key(&x) {
            ans += if x == a[n-1] { n-2 } else { r[&x] } - l[&x] + 1;
        }
    }

    let x = p[n-3] - a[n-2];
    if x <= a[n-2] && r.contains_key(&x) {
        ans += if x == a[n-2] { n-3 } else { r[&x] } - l[&x] + 1;
    }

    ans += if p[n-4] == a[n-3] { 1 } else { 0 };

    writeln!(so, "{}", ans)?;

    Ok(())
}