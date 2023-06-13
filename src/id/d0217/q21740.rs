// BOJ 21740 [Math Game]
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

fn rev(c: char) -> char {
    match c {
        '6' => '9',
        '9' => '6',
        _ => c,
    }
}
fn srev(s: &String) -> String {
    s.chars().rev().map(rev).collect()
}
fn parse(s: String) -> usize {
    s.parse().unwrap()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = (0..n).map(|_| srev(&next::<String>(&mut it))).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| {
        parse(a.clone() + b).cmp(&parse(b.clone() + a))
    });

    let (mut max, mut maxi) = ("0".to_string(), 0);
    for (i, s) in v.iter().enumerate() {
        if s.len() > max.len() {
            max = s.clone();
            maxi = i;
        } else if s.len() == max.len() {
            if parse(s.clone()) > parse(max.clone()) {
                max = s.clone();
                maxi = i;
            }
        }
    }
    v[maxi] = v[maxi].clone() + &v[maxi];
    println!("{}", v.iter().map(srev).collect::<String>());
}
