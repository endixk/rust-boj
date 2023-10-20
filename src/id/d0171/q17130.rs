// BOJ 17130 [Rabbit]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    const INF: i32 = 0x3f3f3f3f;
    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec!['#'; 1005000];
    let mut d = vec![-INF; 1005000];
    for i in 1..=n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            b[i*(m+2)+j+1] = c;
            if c == 'R' { d[i*(m+2)+j+1] = 0; }
        }
    }

    let mut ans = -1;
    for j in 2..=m {
        for i in 1..=n {
            if b[i*(m+2)+j] == '#' { continue; }
            let f = if b[i*(m+2)+j] == 'C' { 1 } else { 0 };
            d[i*(m+2)+j] = d[i*(m+2)+j].max(d[(i-1)*(m+2)+j-1] + f).max(d[i*(m+2)+j-1] + f).max(d[(i+1)*(m+2)+j-1] + f);
            if b[i*(m+2)+j] == 'O' {
                ans = ans.max(d[i*(m+2)+j]);
            }
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
