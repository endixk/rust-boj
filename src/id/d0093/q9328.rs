// BOJ 9328 [Keys]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

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

fn open(map: &mut Vec<Vec<char>>, key: char) {
    let door = (key as u8 - 'a' as u8 + 'A' as u8) as char;
    for i in 1..map.len() - 1 {
        for j in 1..map[i].len() - 1 {
            if map[i][j] == door {
                map[i][j] = '.';
            } else if map[i][j] == key {
                map[i][j] = '.';
            }
        }
    }
}

fn bfs(map: &mut Vec<Vec<char>>, h: usize, w: usize, ix: usize, iy: usize) -> i32 {
    let mut ans = 0;
    let mut visited = vec![vec![false; w+2]; h+2];
    let (dx, dy) = (vec![-1, 0, 1, 0], vec![0, 1, 0, -1]);

    let mut q = VecDeque::new();
    q.push_back((ix, iy));
    while let Some((i, j)) = q.pop_front() {
        if visited[i][j] { continue; }
        visited[i][j] = true;

        match map[i][j] {
            '$' => {
                ans += 1;
                map[i][j] = '.';
            },
            'a'..='z' => {
                open(map, map[i][j]);
                return ans + bfs(map, h, w, i, j);
            },
            _ => {},
        }

        for k in 0..4 {
            let (ni, nj) = (i as i32 + dx[k], j as i32 + dy[k]);
            if ni < 0 || ni >= h as i32 + 2 || nj < 0 || nj >= w as i32 + 2 { continue; }
            let (ni, nj) = (ni as usize, nj as usize);
            if visited[ni][nj] { continue; }
            match map[ni][nj] {
                '*' | 'A'..='Z' => {},
                _ => q.push_back((ni, nj)),
            }
        }
    }

    ans
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let t = next::<usize>(&mut it);
    for _ in 0..t {
        let (h, w) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let mut map = vec![vec!['.'; w+2]; h+2];
        for i in 1..=h {
            next::<String>(&mut it).chars().collect::<Vec<char>>()
                .iter().enumerate().for_each(|(j, &c)| map[i][j+1] = c);
        }

        next::<String>(&mut it).chars().filter(|&c| c != '0').for_each(|c| {
            open(&mut map, c);
        });

        writeln!(so, "{}", bfs(&mut map, h, w, 0, 0)).unwrap();
    }
}
