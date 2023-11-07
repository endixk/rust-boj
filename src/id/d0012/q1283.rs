// BOJ 1283 [Shortcut Keys]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};
fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let mut a = vec![false; 26];
    's: for _ in 0..read(&mut si).parse::<usize>().unwrap() {
        let s = read(&mut si);
        let mut k = None;

        for w in s.split(" ") {
            let c = w.chars().next().unwrap();
            if !a[c.to_lowercase().next().unwrap() as usize - 'a' as usize] {
                a[c.to_lowercase().next().unwrap() as usize - 'a' as usize] = true;
                k = Some(c);
                break;
            }
        }
        if let Some(c) = k {
            let mut flag = false;
            for w in s.split(" ") {
                if flag {
                    write!(so, "{} ", w)?;
                } else if w.chars().next().unwrap() == c {
                    flag = true;
                    write!(so, "[{}]{} ", c, &w[1..])?;
                } else {
                    write!(so, "{} ", w)?;
                }
            }
            writeln!(so)?;
            continue;
        }

        for c in s.chars() {
            if c == ' ' { continue; }
            if !a[c.to_lowercase().next().unwrap() as usize - 'a' as usize] {
                a[c.to_lowercase().next().unwrap() as usize - 'a' as usize] = true;
                writeln!(so, "{}", s.replacen(c, &format!("[{}]", c), 1))?;
                continue 's;
            }
        }

        writeln!(so, "{}", s)?;
    }

    Ok(())
}
