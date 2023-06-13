// BOJ 26976 [Feeding the Cows]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let s = next::<String>(&mut it);
        let s = s.chars().collect::<Vec<char>>();

        let mut ans = vec!['.'; n];
        let mut cnt = 0;
        let (mut g, mut h) = (0, 0);
        while g < n || h < n {
            if g == h {
                if s[g] == 'G' {
                    let mut x = (n-1).min(g + k);
                    while ans[x] != '.' { x -= 1; }
                    ans[x] = 'G';
                    g = x + k + 1;
                    cnt += 1;
                } else {
                    let mut x = (n-1).min(h + k);
                    while ans[x] != '.' { x -= 1; }
                    ans[x] = 'H';
                    h = x + k + 1;
                    cnt += 1;
                }
            } else if g < h {
                if s[g] == 'G' {
                    let mut x = (n-1).min(g + k);
                    while ans[x] != '.' { x -= 1; }
                    ans[x] = 'G';
                    g = x + k + 1;
                    cnt += 1;
                } else { g += 1; }
            } else {
                if s[h] == 'H' {
                    let mut x = (n-1).min(h + k);
                    while ans[x] != '.' { x -= 1; }
                    ans[x] = 'H';
                    h = x + k + 1;
                    cnt += 1;
                } else { h += 1; }
            }
        }

        writeln!(so, "{cnt}").ok();
        writeln!(so, "{}", ans.iter().collect::<String>()).ok();
    }
}
