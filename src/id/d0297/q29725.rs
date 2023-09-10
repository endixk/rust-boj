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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut ans = 0;
    for _ in 0..8 {
        for c in next::<String>(&mut it).chars() {
            match c {
                'K' | 'k' => (),
                'P' => ans += 1,
                'p' => ans -= 1,
                'N' | 'B' => ans += 3,
                'n' | 'b' => ans -= 3,
                'R' => ans += 5,
                'r' => ans -= 5,
                'Q' => ans += 9,
                'q' => ans -= 9,
                _ => (),
            }
        }
    }
    writeln!(so, "{}", ans).unwrap();
}