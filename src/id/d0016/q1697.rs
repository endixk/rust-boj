// BOJ 1697 [Hide and Seek]
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
    q.push_back(src);
    v[src] = true;

    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            if cur == dst {
                return cnt;
            }
            if cur > 0 && !v[cur - 1] {
                q.push_back(cur - 1);
                v[cur - 1] = true;
            }
            if cur < MAX - 1 && !v[cur + 1] {
                q.push_back(cur + 1);
                v[cur + 1] = true;
            }
            if cur * 2 < MAX && !v[cur * 2] {
                q.push_back(cur * 2);
                v[cur * 2] = true;
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
