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

    let (n, h) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let s = next::<String>(&mut it);
    let s = s.as_bytes();
    if h > 3 {
        if n < 4 {
            let mut cnt = 0;
            if s.contains(&b'S') { cnt += 1; }
            if s.contains(&b'R') { cnt += 1; }
            if s.contains(&b'W') { cnt += 1; }
            writeln!(so, "{}", n - cnt).ok();
        }
        else { writeln!(so, "-1").ok(); }
    }
    else if h == 1 { writeln!(so, "0").ok(); }
    else if h == 2 {
        let mut ans = 0;
        let mut i = 0;
        while i < n-1 {
            if s[i] == s[i+1] { ans += 1; i += 2; }
            else { i += 1; }
        }
        writeln!(so, "{}", ans).ok();
    } else {
        let mut ans = n;
        let go = vec![
            vec![b'S', b'R', b'W'],
            vec![b'S', b'W', b'R'],
            vec![b'R', b'S', b'W'],
            vec![b'R', b'W', b'S'],
            vec![b'W', b'S', b'R'],
            vec![b'W', b'R', b'S'],
        ];
        for v in go {
            let mut i = 0;
            let mut cnt = 0;
            while i < n {
                if s[i] != v[i%3] { cnt += 1; }
                i += 1;
            }
            ans = ans.min(cnt);
        }
        writeln!(so, "{}", ans).ok();
    }
}