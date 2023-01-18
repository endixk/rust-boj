// BOJ 2753 [Leap Year]
// Supported by GitHub Copilot

use std::io;

pub fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<i16>().unwrap();
    println!("{}",
             (x % 4 == 0 && (x % 100 != 0 || x % 400 == 0))
                 .then(|| "1")
                 .unwrap_or("0")
    )
}
