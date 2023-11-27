// BOJ 30776 [Schedule]
// Supported by GitHub Copilot

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

#[inline] fn week(s: &str) -> usize {
    match s {
        "Monday" => 0, "Tuesday" => 1, "Wednesday" => 2, "Thursday" => 3, "Friday" => 4, "Saturday" => 5, "Sunday" => 6,
        _ => unreachable!()
    }
}
#[inline] fn month(s: &str) -> usize {
    match s {
        "January" => 0, "February" => 1, "March" => 2, "April" => 3,
        "May" => 4, "June" => 5, "July" => 6, "August" => 7,
        "September" => 8, "October" => 9, "November" => 10, "December" => 11,
        _ => unreachable!()
    }
}
const DAYS: [u8; 12] = [31,28,31,30,31,30,31,31,30,31,30,31];
const LDAYS: [u8; 12] = [31,29,31,30,31,30,31,31,30,31,30,31];
#[inline] fn leap(y: u32) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let y = next::<u32>(&mut it);
    let h = (0..n).map(|_| (next::<u8>(&mut it), month(next::<String>(&mut it).as_str()))).collect::<Vec<_>>();
    let mut w = week(next::<String>(&mut it).as_str());

    let mut cnt = vec![0; 7];
    let (mut m, mut d) = (0, 0);
    while m < 12 {
        cnt[w] += if h.contains(&(d+1, m)) { 0 } else { 1 };
        w = (w + 1) % 7;
        d += 1;
        if d == if leap(y) { LDAYS[m] } else { DAYS[m] } {
            d = 0;
            m += 1;
        }
    }

    let (mut max, mut mxd, mut min, mut mnd) = (0, 0, 100, 0);
    for (i, &c) in cnt.iter().enumerate() {
        if c > max { max = c; mxd = i; }
        if c < min { min = c; mnd = i; }
    }
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    writeln!(so, "{} {}", days[mxd], days[mnd])?;

    Ok(())
}
