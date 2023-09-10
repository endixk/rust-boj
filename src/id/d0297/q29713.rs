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

    let mut cnt = [0; 26];
    for c in s.split_ascii_whitespace().skip(1).next().unwrap().chars() {
        cnt[(c as u8 - b'A') as usize] += 1;
    }
    let mut ans = 9999;
    for c in vec!['B', 'O', 'N', 'Z', 'S', 'I', 'L', 'V'] {
        ans = ans.min(cnt[(c as u8 - b'A') as usize]);
    }
    for c in vec!['R', 'E'] {
        ans = ans.min(cnt[(c as u8 - b'A') as usize] / 2);
    }
    writeln!(so, "{}", ans).unwrap();
}