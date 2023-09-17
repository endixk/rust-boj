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

    let (_, n, _) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let ink = next::<String>(&mut it);
    let ink = ink.chars().collect::<Vec<_>>();
    let mut b = vec![vec![-1; n]; n];

    let (mut x, mut y) = (0, 0);
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            match c {
                '@' => { x = i; y = j; },
                '#' => { b[i][j] = 0; },
                _ => (),
            }
        }
    }

    let cmd = next::<String>(&mut it);
    let (mut k, mut m) = (0, 1i32);
    for c in cmd.chars() {
        match c {
            'U' => if x > 0 && b[x-1][y] < 0 { x -= 1; },
            'D' => if x < n-1 && b[x+1][y] < 0 { x += 1; },
            'L' => if y > 0 && b[x][y-1] < 0 { y -= 1; },
            'R' => if y < n-1 && b[x][y+1] < 0 { y += 1; },
            'j' => k += 1,
            'J' => {
                for i in 0..n { for j in 0..n {
                    if b[i][j] >= 0 {
                        let dx = if i > x { i - x } else { x - i };
                        let dy = if j > y { j - y } else { y - j };
                        if dx + dy <= k { b[i][j] = m; }
                    }
                }}
                k = 0;
                m += 1;
                if m as usize > ink.len() { m = 1; }
            }
            _ => (),
        }
    }

    for i in 0..n {
        for j in 0..n {
            if b[i][j] < 0 {
                if i == x && j == y { write!(so, "@").ok(); }
                else { write!(so, ".").ok(); }
            } else if b[i][j] == 0 { write!(so, "#").ok(); }
            else { write!(so, "{}", ink[b[i][j] as usize - 1]).ok(); }
        }
        writeln!(so).ok();
    }
}