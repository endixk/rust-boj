// BOJ 30686 [Assignments]
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

fn next_permutation<T>(arr: &mut [T]) -> bool where T: Ord {
    use std::cmp::Ordering;
    let la = match arr.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => { arr.reverse();return false; }
    };
    let sw = arr[la + 1..]
        .binary_search_by(|n| match arr[la].cmp(n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        }).unwrap_err();
    arr.swap(la, la + sw);
    arr[la + 1..].reverse();
    true
}

fn solve(ord: &[usize], n: usize, m: usize, d: &[i32], a: &Vec<Vec<usize>>) -> usize {
    let mut ret = 0;
    let mut k = vec![-1; n];
    for i in 0..m {
        for j in 0..n {
            if k[j] != -1 && k[j] + d[j] == i as i32 { k[j] = -1; }
        }
        let t = ord[i];
        for &j in a[t].iter() {
            if k[j] == -1 { k[j] = i as i32; ret += 1; }
        }
    }
    ret
}
fn go(n: usize, m: usize, d: &[i32], a: &Vec<Vec<usize>>) -> usize {
    let mut ord = (0..m).collect::<Vec<_>>();
    let mut ret = solve(&ord, n, m, d, a);
    while next_permutation(&mut ord) {
        ret = ret.min(solve(&ord, n, m, d, a));
    }
    ret
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let d = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let a = (0..m).map(|_| {
        let k = next::<usize>(&mut it);
        (0..k).map(|_| next::<usize>(&mut it) - 1).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    writeln!(so, "{}", go(n, m, &d, &a))?;

    Ok(())
}
