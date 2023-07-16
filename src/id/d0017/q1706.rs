// BOJ 1706 [Crossword Puzzle]
// Supported by GitHub Copilot

use std::io::{self, BufRead};

pub fn main() {
    let board = io::stdin().lock().lines().skip(1).map(|s| s.unwrap()).collect::<Vec<_>>();
    let mut words = Vec::new();
    board.iter().for_each(|s| s.split('#').filter(|&s| s.len() > 1).for_each(|s| words.push(s.to_string())));
    (0..board[0].len()).map(|i| board.iter().map(|s| s.chars().nth(i).unwrap()).collect::<String>()).for_each(|s| s.split('#').filter(|&s| s.len() > 1).for_each(|s| words.push(s.to_string())));
    println!("{}", words.iter().skip(1).fold(words[0].clone(), |a, b| if a > b.clone() { b.clone() } else { a }));
}
