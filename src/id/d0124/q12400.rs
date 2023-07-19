// BOJ 12400 [Speaking in Tongues]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
const MAP: [char; 26] = [
    'y', 'h', 'e', 's', 'o', 'c', 'v', 'x', 'd', 'u',
    'i', 'g', 'l', 'b', 'k', 'r', 'z', 't', 'n', 'w',
    'j', 'p', 'f', 'm', 'a', 'q',
];
pub fn main() {
    let mut i = 1;
    for s in io::stdin().lock().lines().skip(1) {
        println!("Case #{i}: {}", s.unwrap().chars().map(|c| {
            if c == ' ' { ' ' } else { MAP[(c as u8 - b'a') as usize] }
        }).collect::<String>());
        i += 1;
    }
}
