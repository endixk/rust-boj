// BOJ 14939 [Lights Off]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn press(a: &mut [[bool; 10]; 10], i: usize, j: usize) {
    a[i][j] = !a[i][j];
    if i > 0 { a[i - 1][j] = !a[i - 1][j]; }
    if i < 9 { a[i + 1][j] = !a[i + 1][j]; }
    if j > 0 { a[i][j - 1] = !a[i][j - 1]; }
    if j < 9 { a[i][j + 1] = !a[i][j + 1]; }
}

fn go(ori: &[[bool; 10]; 10], bit: u16) -> Option<u8> {
    let mut ret = 0;
    let mut a = ori.clone();

    for j in 0..10 {
        if bit & (1 << j) != 0 {
            press(&mut a, 0, j);
            ret += 1;
        }
    }
    for i in 1..10 { for j in 0..10 {
        if a[i - 1][j] {
            press(&mut a, i, j);
            ret += 1;
        }
    }}

    for i in 0..10 { for j in 0..10 {
        if a[i][j] { return None; }
    }}
    Some(ret)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut ori = [[false; 10]; 10];
    for i in 0..10 {
        let s = it.next().unwrap();
        for (j, c) in s.chars().enumerate() {
            ori[i][j] = c == 'O';
        }
    }

    let mut ans = 101;
    for bit in 0..1024 {
        if let Some(x) = go(&ori, bit) {
            ans = ans.min(x);
        }
    }
    println!("{}", if ans > 100 { -1 } else { ans as i8 });
}
