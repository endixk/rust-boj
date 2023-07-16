use std::io::{self, BufRead};
pub fn main() {
    println!("{}", io::stdin().lock().lines().next().unwrap().unwrap().into_bytes().iter().fold(0, |acc, &x| acc * 26 + x as i64 - 64));
}
