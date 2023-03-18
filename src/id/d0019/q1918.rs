// BOJ 1918 [Postfix Expression]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn op(st: &mut Vec<char>, c: char) -> String {
    let mut buf = String::new();
    if st.is_empty(){
        st.push(c);
        return buf
    }
    match c {
        '+' | '-' => {
            while let Some(&x) = st.last() {
                if x == '(' {
                    break;
                }
                buf.push(st.pop().unwrap());
            }
            st.push(c);
        },
        '*' | '/' => {
            while let Some(&x) = st.last() {
                if x == '(' || x == '+' || x == '-' {
                    break;
                }
                buf.push(st.pop().unwrap());
            }
            st.push(c);
        },
        ')' => {
            while let Some(&x) = st.last() {
                if x == '(' {
                    st.pop();
                    break;
                }
                buf.push(st.pop().unwrap());
            }
        },
        _ => st.push(c),
    }
    buf
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);

    let mut st = Vec::new();
    for c in s.trim().chars() {
        match c {
            'A'..='Z' => write!(so, "{}", c).unwrap(),
            _ => write!(so, "{}", op(&mut st, c)).unwrap(),
        }
    }
    while let Some(_) = st.last() {
        write!(so, "{}", st.pop().unwrap()).unwrap();
    }
}
