use std::io::{self, BufRead, Write};
fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);

    let n = s.parse::<usize>().unwrap();
    let (mut a, mut b) = (vec![], vec![]);
    for _ in 0..n {
        let s = read(&mut si);
        if s.contains("/") {
            b.push(s.split("/").skip(1).next().unwrap().parse::<i32>().unwrap());
        } else { a.push(s); }
    }

    a.sort_unstable_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    b.sort_unstable();
    for s in a { writeln!(so, "{}", s).unwrap(); }
    for i in b { writeln!(so, "boj.kr/{}", i).unwrap(); }
}