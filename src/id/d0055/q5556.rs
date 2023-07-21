use std::io::{self, Read, Write};
pub fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_ascii_whitespace();
    let mut so = io::BufWriter::new(io::stdout().lock());
    let n: i32 = it.next().unwrap().parse().unwrap();
    for _ in 0..it.next().unwrap().parse().unwrap() {
        let (i, j): (i32, i32) = (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap());
        writeln!(so, "{}", (i-1).min(j-1).min(n-i).min(n-j)%3+1).ok();
    }
}
