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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, u, l) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    if n >= 1000 && (u >= 8000 || l >= 260) { println!("Very Good")}
    else if n >= 1000 { println!("Good")}
    else { println!("Bad")}
}