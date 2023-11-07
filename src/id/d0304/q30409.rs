// BOJ 30409 [Telephone Poles]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let h = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut cf = vec![0; n];
    let mut st = vec![];
    st.push((h[n-1], n-1));
    for i in (0..n-1).rev() {
        while !st.is_empty() && st.last().unwrap().0 < h[i] {
            st.pop();
        }
        if !st.is_empty() {
            let (x, j) = *st.last().unwrap();
            cf[i] = cf[j] + (j - i).pow(2) + (x - h[i]).pow(2);
        }
        st.push((h[i], i));
    }

    let mut cb = vec![0; n];
    let mut st = vec![];
    st.push((h[0], 0));
    for i in 1..n {
        while !st.is_empty() && st.last().unwrap().0 < h[i] {
            st.pop();
        }
        if !st.is_empty() {
            let (x, j) = *st.last().unwrap();
            cb[i] = cb[j] + (i - j).pow(2) + (x - h[i]).pow(2);
        }
        st.push((h[i], i));
    }

    for _ in 0..next(&mut it) {
        let p = next::<usize>(&mut it);
        writeln!(so, "{}", cf[p-1] + cb[p-1])?;
    }

    Ok(())
}
