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

fn switch(c: char) -> char {
    return match c {
        'R' => 'G',
        'G' => 'B',
        'B' => 'R',
        _ => unreachable!(),
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (_, s) = (next::<usize>(&mut it), next::<String>(&mut it));
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = 0x3f3f3f3f;

    // make R
    let mut t = s.clone();
    let mut cnt = 0;
    for i in (2..s.len()).rev() {
        if t[i] == 'R' { continue; }
        else if t[i] == 'G' {
            cnt += 2;
            t[i-1] = switch(switch(t[i-1]));
            t[i-2] = switch(switch(t[i-2]));
        } else {
            cnt += 1;
            t[i-1] = switch(t[i-1]);
            t[i-2] = switch(t[i-2]);
        }
    }
    if t[0] == 'R' && t[1] == 'R' {
        ans = ans.min(cnt);
    }

    // make G
    let mut t = s.clone();
    let mut cnt = 0;
    for i in (2..s.len()).rev() {
        if t[i] == 'G' { continue; }
        else if t[i] == 'B' {
            cnt += 2;
            t[i-1] = switch(switch(t[i-1]));
            t[i-2] = switch(switch(t[i-2]));
        } else {
            cnt += 1;
            t[i-1] = switch(t[i-1]);
            t[i-2] = switch(t[i-2]);
        }
    }
    if t[0] == 'G' && t[1] == 'G' {
        ans = ans.min(cnt);
    }

    // make B
    let mut t = s.clone();
    let mut cnt = 0;
    for i in (2..s.len()).rev() {
        if t[i] == 'B' { continue; }
        else if t[i] == 'R' {
            cnt += 2;
            t[i-1] = switch(switch(t[i-1]));
            t[i-2] = switch(switch(t[i-2]));
        } else {
            cnt += 1;
            t[i-1] = switch(t[i-1]);
            t[i-2] = switch(t[i-2]);
        }
    }
    if t[0] == 'B' && t[1] == 'B' {
        ans = ans.min(cnt);
    }

    writeln!(so, "{}", if ans == 0x3f3f3f3f { -1 } else { ans as i32 })?;

    Ok(())
}