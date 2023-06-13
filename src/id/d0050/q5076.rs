// BOJ 5076 [Web Pages]
// Supported by GitHub Copilot

use std::io::{self, BufRead, Write};

fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    loop {
        let s = read(&mut si);
        if s == "#" { break; }

        let s = s.as_bytes();
        let mut st = Vec::new();
        let mut legal = true;
        for mut i in 0..s.len() {
            if s[i] == b'<' {
                let mut tag = String::new();
                while s[i+1] != b'>' && s[i+1] != b' ' {
                    tag.push(s[i+1] as char);
                    i += 1;
                }
                if tag.starts_with("/") {
                    if st.is_empty() {
                        legal = false;
                        break;
                    }
                    if st.pop().unwrap() != tag[1..].to_string() {
                        legal = false;
                        break;
                    }
                } else {
                    while s[i+1] != b'>' {
                        i += 1;
                    }
                    if s[i] != b'/' {
                        st.push(tag);
                    }
                }
            } else {
                continue;
            }
        }

        writeln!(so, "{}", if legal && st.is_empty() { "legal" } else { "illegal" }).ok();
    }
}
