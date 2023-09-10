use std::io::{self, BufRead, Write};
fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let v = read(&mut si).split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let (j, n) = (v[0], v[1]);

    let mut ans = 0;
    for _ in 0..n {
        let mut k = 0;
        for c in read(&mut si).chars() {
            match c {
                'A'..='Z' => k += 4,
                'a'..='z' => k += 2,
                '0'..='9' => k += 2,
                ' ' => k += 1,
                _ => unreachable!()
            }
        }
        if k <= j { ans += 1; }
    }
    writeln!(so, "{}", ans).unwrap();
}