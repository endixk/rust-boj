// BOJ 1236 [Defend the Castle]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

pub fn main() {
    let mut board = [[false; 50]; 50];
    let (mut n, mut m) = (0, 0);
    io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .enumerate()
        .for_each(|(i, x)| {
            if i == 0 {
                let mut iter = x.split_whitespace();
                n = iter.next().unwrap().parse::<usize>().unwrap();
                m = iter.next().unwrap().parse::<usize>().unwrap();
                return;
            }
            x.chars().enumerate().for_each(|(j, y)| {
                if y == 'X' { board[i - 1][j] = true; }
            });
        });

    let (mut row, mut col) = (0, 0);
    for i in 0..n { row += if board[i].iter().all(|&x| !x) { 1 } else { 0 }; }
    for i in 0..m { col += if board.iter().all(|x| !x[i]) { 1 } else { 0 }; }
    println!("{}", row.max(col));
}
