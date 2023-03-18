// BOJ 13549 [Hide and Seek 3]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

const MAX: usize = 100001;

fn bfs(src: usize, dst: usize) -> usize {
    let mut cnt = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut v = vec![false; MAX];

    let mut nxt = src;
    while nxt < MAX {
        q.push_back(nxt);
        v[nxt] = true;
        nxt *= 2;
        if nxt == 0 { break; }
    }

    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            if cur == dst {
                return cnt;
            }
            if cur > 0 && !v[cur - 1] {
                let mut nxt = cur - 1;
                while nxt < MAX {
                    if !v[nxt] {
                        q.push_back(nxt);
                        v[nxt] = true;
                    }
                    nxt *= 2;
                    if nxt == 0 { break; }
                }
            }
            if cur < MAX - 1 && !v[cur + 1] {
                let mut nxt = cur + 1;
                while nxt < MAX {
                    if !v[nxt] {
                        q.push_back(nxt);
                        v[nxt] = true;
                    }
                    nxt *= 2;
                }
            }
        }
        cnt += 1;
    }
    cnt
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let v = read(&mut si).split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    writeln!(so, "{}", bfs(v[0], v[1])).unwrap();
}
