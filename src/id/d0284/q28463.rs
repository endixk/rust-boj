// BOJ 28463 [Toe Jumps]
// Supported by GitHub Copilot

use std::io::BufRead;
pub fn main() {
    let v = std::io::stdin().lock().lines().take(3).map(|x| x.unwrap()).collect::<Vec<_>>();
    let s = format!("{}{}", v[1], v[2])
        .replace(".", "00").replace("I", "01").replace("O", "10").replace("P", "11");
    let x = usize::from_str_radix(&s, 2).unwrap();
    match v[0].as_str() {
        "S" if x == 44 => println!("T"),
        "S" if x == 67 => println!("F"),
        "S" if x == 131 => println!("Lz"),
        "E" if x == 131 => println!("T"),
        "E" if x == 52 => println!("F"),
        "E" if x == 56 => println!("Lz"),
        "N" if x == 56 => println!("T"),
        "N" if x == 193 => println!("F"),
        "N" if x == 194 => println!("Lz"),
        "W" if x == 194 => println!("T"),
        "W" if x == 28 => println!("F"),
        "W" if x == 44 => println!("Lz"),
        _ => println!("?"),
    }
}
