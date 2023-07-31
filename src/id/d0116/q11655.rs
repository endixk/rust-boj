use std::io::{self, BufRead};
pub fn main() {
    let s = io::stdin().lock().lines().next().unwrap().unwrap();
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            print!("{}", if c > 'Z' {
                (c as u8 + 13 - b'a') % 26 + b'a' as u8
            } else {
                (c as u8 + 13 - b'A') % 26 + b'A' as u8
            } as char);
        } else {
            print!("{}", c);
        }
    }
}
