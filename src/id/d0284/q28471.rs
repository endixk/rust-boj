// BOJ 28471 [Broken W Key]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

const DX: [i32; 7] = [0, 1, 1, 0, -1, -1, -1];
const DY: [i32; 7] = [1, 1, -1, -1, -1, 0, 1];
use std::collections::VecDeque;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut board = vec![vec!['#'; n]; n];
    let (mut x, mut y) = (0, 0);
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            board[i][j] = c;
            if c == 'F' { x = i; y = j; }
        }
    }

    let mut q = VecDeque::new();
    q.push_back((x, y));
    let mut ans = 0;
    while let Some((x, y)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (x, y) = (x as i32 + dx, y as i32 + dy);
            if x < 0 || x >= n as i32 || y < 0 || y >= n as i32 { continue; }
            let (x, y) = (x as usize, y as usize);
            if board[x][y] == '.' {
                board[x][y] = '#';
                ans += 1;
                q.push_back((x, y));
            }
        }
    }
    println!("{}", ans);
}
