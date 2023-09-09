use std::io::{self, Read, Write};
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

use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut board = vec![vec!['0'; m]; n];
    let (mut si, mut sj, mut hi, mut hj) = (0, 0, 0, 0);
    let mut dst = HashMap::new();
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            board[i][j] = c;
            if c == 'S' { si = i; sj = j; }
            if c == 'H' { hi = i; hj = j; }
            if c == 'F' { dst.insert((i, j), 0x3f3f3f3f); }
        }
    }

    let mut q = VecDeque::new();
    q.push_back((si, sj));
    let mut vis = vec![vec![false; m]; n];
    vis[si][sj] = true;
    let mut d = 1;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let (x, y) = q.pop_front().unwrap();
            for (dx, dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
                let (nx, ny) = (nx as usize, ny as usize);
                if board[nx][ny] == 'D' || vis[nx][ny] { continue; }
                if board[nx][ny] == 'F' { dst.insert((nx, ny), d); }
                vis[nx][ny] = true;
                q.push_back((nx, ny));
            }
        }
        d += 1;
    }

    q.push_back((hi, hj));
    vis = vec![vec![false; m]; n];
    vis[hi][hj] = true;
    d = 1;
    let mut ans = 0x3f3f3f3f;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let (x, y) = q.pop_front().unwrap();
            for (dx, dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
                let (nx, ny) = (nx as usize, ny as usize);
                if board[nx][ny] == 'D' || vis[nx][ny] { continue; }
                if board[nx][ny] == 'F' { ans = ans.min(*dst.get(&(nx, ny)).unwrap() + d); }
                vis[nx][ny] = true;
                q.push_back((nx, ny));
            }
        }
        d += 1;
    }

    writeln!(so, "{}", if ans == 0x3f3f3f3f { "-1".to_string() } else { ans.to_string() }).ok();
}