use std::io::{self, BufRead, Write};
fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}
fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let rick = vec![
        "Never gonna give you up",
        "Never gonna let you down",
        "Never gonna run around and desert you",
        "Never gonna make you cry",
        "Never gonna say goodbye",
        "Never gonna tell a lie and hurt you",
        "Never gonna stop",
    ];
    let mut flag = true;
    for _ in 0..read(&mut si).parse::<usize>().unwrap() {
        let s = read(&mut si);
        if !rick.contains(&s.as_str()) {
            flag = false; break;
        }
    }
    writeln!(so, "{}", if flag { "No" } else { "Yes" }).unwrap();
}