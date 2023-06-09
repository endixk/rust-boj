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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = (0..n).map(|_| {
        let s = next::<String>(&mut it);
        let s = srev(&s);
        (s.parse::<u64>().unwrap(), s.len() as u32)
    }).collect::<Vec<_>>();
    v.sort_unstable_by(|&(a, al), &(b, bl)| {
        (b * 10u64.pow(al) + a).cmp(&(a * 10u64.pow(bl) + b))
    });

    let (mut max, mut maxl, mut maxi) = (0, 0, 0);
    for (i, &(n, l)) in v.iter().enumerate() {
        if l > maxl { max = n; maxl = l; maxi = i; }
        else if l == maxl && n > max { max = n; maxi = i; }
    }
    v[maxi].0 = max * 10u64.pow(maxl) + max;
    v[maxi].1 *= 2;

    let mut ans = String::new();
    for (n, l) in v { ans += &format!("{:0>1$}", n, l as usize); }
    println!("{}", srev(&ans));
}
