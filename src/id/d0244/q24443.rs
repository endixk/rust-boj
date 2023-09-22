// BOJ 24443 [Selection Algorithm 4]
// Supported by GitHub Copilot

use std::io::{self, Read};
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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![0; n+1];
    for i in 1..=n {
        a[i] = next::<u32>(&mut it);
    }
    for _ in 0..q {
        match next::<u8>(&mut it) {
            1 => {
                let (i, j, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
                let mut v = a[i..=j].to_vec();
                v.select_nth_unstable(k-1);
                println!("{}", v[k-1]);
            },
            _ => {
                let (i, j) = (next::<usize>(&mut it), next::<usize>(&mut it));
                a.swap(i, j);
            }
        }
    }

    Ok(())
}
