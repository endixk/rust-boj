// BOJ 2239 [Sudoku]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn fill(
    board: &mut Vec<Vec<u8>>,
    rmem: &mut Vec<Vec<bool>>,
    cmem: &mut Vec<Vec<bool>>,
    bmem: &mut Vec<Vec<bool>>,
    i: usize, j: usize) -> bool {
    if i == 9 { return true; }
    if j == 9 { return fill(board, rmem, cmem, bmem, i + 1, 0); }
    if board[i][j] != 0 { return fill(board, rmem, cmem, bmem, i, j + 1); }
    for v in 1..=9 {
        if rmem[i][v] || cmem[j][v] || bmem[i / 3 * 3 + j / 3][v] { continue; }
        board[i][j] = v as u8;
        rmem[i][v] = true;
        cmem[j][v] = true;
        bmem[i / 3 * 3 + j / 3][v] = true;
        if fill(board, rmem, cmem, bmem, i, j + 1) { return true; }
        board[i][j] = 0;
        rmem[i][v] = false;
        cmem[j][v] = false;
        bmem[i / 3 * 3 + j / 3][v] = false;
    }
    false
}


pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let mut board = vec![vec![0; 9]; 9];
    for i in 0..9 {
        let s = next::<String>(&mut it);
        for j in 0..9 {
            board[i][j] = s.as_bytes()[j] - b'0';
        }
    }

    let mut rmem = vec![vec![false; 10]; 9];
    let mut cmem = vec![vec![false; 10]; 9];
    let mut bmem = vec![vec![false; 10]; 9];
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == 0 { continue; }
            rmem[i][board[i][j] as usize] = true;
            cmem[j][board[i][j] as usize] = true;
            bmem[i / 3 * 3 + j / 3][board[i][j] as usize] = true;
        }
    }

    fill(&mut board, &mut rmem, &mut cmem, &mut bmem, 0, 0);
    for i in 0..9 {
        for j in 0..9 {
            print!("{}", board[i][j]);
        }
        println!();
    }
}
