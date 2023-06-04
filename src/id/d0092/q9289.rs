// BOJ 9289 [Morse Code]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

const MC: [&str; 26] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---",
    "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-",
    "..-", "...-", ".--", "-..-", "-.--", "--.."
];
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    for (i, s) in read(&mut si).split("\n").enumerate().skip(1) {
        if s.is_empty() { break; }
        write!(so, "Case {}: ", i).ok();
        for c in s.split_whitespace() {
            write!(so, "{}", (b'A' + MC.iter().position(|&x| x == c).unwrap() as u8) as char).ok();
        }
        writeln!(so).ok();
    }
}
