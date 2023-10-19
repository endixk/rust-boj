// BOJ 30209 [Trench Warfare]
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

use std::collections::BTreeSet;
fn insert(s: &mut BTreeSet<(i64, usize)>, sum: &mut i64, lim: i64,
          t: &mut BTreeSet<(i64, usize)>, q: &(i64, usize)) {
    s.insert(*q);
    *sum += q.0;
    while !s.is_empty() && *sum > lim {
        let (i, j) = s.pop_last().unwrap();
        *sum -= i;
        t.insert((i, j));
    }
}
fn remove(s: &mut BTreeSet<(i64, usize)>, sum: &mut i64, lim: i64,
          t: &mut BTreeSet<(i64, usize)>, q: &(i64, usize)) {
    if s.contains(q) {
        s.remove(q);
        *sum -= q.0;
        while !t.is_empty() && *sum + t.first().unwrap().0 <= lim {
            let (i, j) = t.pop_first().unwrap();
            s.insert((i, j));
            *sum += i;
        }
    } else {
        t.remove(q);
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<i64>(&mut it), next::<i64>(&mut it));
    let a = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();

    let mut s = BTreeSet::new();
    let mut t = BTreeSet::new();
    let (mut sum, lim) = (0, m);

    let (mut i, mut x, mut c) = (0, 0, 0);
    for (i, &x) in a.iter().enumerate() {
        insert(&mut s, &mut sum, lim, &mut t, &(x+1, i));
    }
    let mut ans = s.len();

    for j in 1..=n {
        x += a[j-1];
        c += 1;
       remove(&mut s, &mut sum, lim, &mut t, &(a[j-1]+1, j-1));
        while x > k && i < j {
            x -= a[i];
            c -= 1;
            insert(&mut s, &mut sum, lim, &mut t, &(a[i]+1, i));
            i += 1;
        }
        ans = ans.max(s.len() + c);
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
