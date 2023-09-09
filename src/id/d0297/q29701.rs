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

const MORSE_ALPH: [&str; 26] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-",
    ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-",
    ".--", "-..-", "-.--", "--.."
];
const MORSE_NUM: [&str; 10] = [
    "-----", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----."
];
const MORSE_PUNC: [&str; 6] = [
    "--..--", ".-.-.-", "..--..", "---...", "-....-", ".--.-."
];
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    for _ in 0..n {
        let s = next::<String>(&mut it);
        let k = match s.len() {
            5 => MORSE_NUM.iter().position(|&x| x == s).unwrap(),
            6 => MORSE_PUNC.iter().position(|&x| x == s).unwrap(),
            _ => MORSE_ALPH.iter().position(|&x| x == s).unwrap(),
        };
        match s.len() {
            5 => write!(so, "{}", (b'0' + k as u8) as char).unwrap(),
            6 => write!(so, "{}", match k {
                0 => ',',
                1 => '.',
                2 => '?',
                3 => ':',
                4 => '-',
                _ => '@',
            }).unwrap(),
            _ => write!(so, "{}", (b'A' + k as u8) as char).unwrap(),
        }
    }
    writeln!(so).unwrap();
}