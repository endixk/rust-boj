// BOJ 12851 [Hide and Seek 2]
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

fn bfs(src: usize, dst: usize) -> (usize, usize) {
    let mut cnt = 0;
    let mut q: VecDeque<usize> = VecDeque::new();
    let mut v = vec![false; MAX];
    let mut c = vec![0; MAX];
    q.push_back(src);
    c[src] = 1;

    while !q.is_empty() {
        let size = q.len();
        let mut nxt = Vec::new();

        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            if cur == dst {
                return (cnt, c[cur]);
            }
            if cur > 0 && !v[cur-1] {
                c[cur-1] += c[cur];
                nxt.push(cur-1);
            }
            if cur < MAX-1 && !v[cur+1] {
                c[cur+1] += c[cur];
                nxt.push(cur+1);
            }
            if cur*2 < MAX && !v[cur*2] {
                c[cur*2] += c[cur];
                nxt.push(cur*2);
            }
        }

        for i in nxt {
            if !v[i] {
                q.push_back(i);
                v[i] = true;
            }
        }
        cnt += 1;
    }

    (cnt, 0)
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let v = read(&mut si).split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let (t, c) = bfs(v[0], v[1]);
    writeln!(so, "{}\n{}", t, c).unwrap();
}
