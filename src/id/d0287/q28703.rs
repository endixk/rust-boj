use std::io::{self, Read};
use std::collections::BinaryHeap;
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
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut pq = BinaryHeap::new();
    let mut min = 0;
    for _ in 0..n {
        let x = -next::<i64>(&mut it);
        if x < min { min = x; }
        pq.push(x);
    }

    let mut ans = i32::MAX as i64;
    let ori = min;
    loop {
        let x = pq.pop().unwrap();
        if x - min < ans { ans = x - min; }
        if x == ori { break; }
        if x*2 < min { min = x*2; }
        pq.push(x*2);
    }
    println!("{}", ans);
}