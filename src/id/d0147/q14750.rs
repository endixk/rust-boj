// BOJ 14750 [Jerry and Tom]
// Supported by GitHub Copilot

use std::cmp::{min, max};
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)] struct Point { x: i64, y: i64 }
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if t > 0 { 1 }
    else if t < 0 { -1 }
    else { 0 }
}

struct Line { a: Point, b: Point }
fn intersect(l1: &Line, l2: &Line) -> bool {
    let ab = ccw(&l1.a, &l1.b, &l2.a) * ccw(&l1.a, &l1.b, &l2.b);
    let cd = ccw(&l2.a, &l2.b, &l1.a) * ccw(&l2.a, &l2.b, &l1.b);
    if ab == 0 && cd == 0 {
        let p1 = min(&l1.a, &l1.b);
        let p2 = max(&l1.a, &l1.b);
        let p3 = min(&l2.a, &l2.b);
        let p4 = max(&l2.a, &l2.b);
        return if p1 == p3 || p1 == p4 || p2 == p3 || p2 == p4 { true }
        else { p2 >= p3 && p4 >= p1 }
    }
    ab <= 0 && cd <= 0
}

use std::collections::VecDeque;
const MAX: usize = 333;
const INF: i32 = 0x3f3f3f3f;
fn dinic_bfs(lvl: &mut Vec<i32>,
             adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, flo: &Vec<Vec<i32>>,
             src: usize, snk: usize) -> bool {
    lvl.iter_mut().for_each(|x| *x = -1);
    lvl[src] = 0;
    let mut q = VecDeque::new();
    q.push_back(src);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if lvl[v] == -1 && cap[u][v] - flo[u][v] > 0 {
                lvl[v] = lvl[u] + 1;
                q.push_back(v);
            }
        }
    }
    lvl[snk] != -1
}
fn dinic_dfs(flo: &mut Vec<Vec<i32>>, wrk: &mut Vec<usize>,
             adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, lvl: &Vec<i32>,
             cur: usize, snk: usize, f: i32) -> Option<i32> {
    if cur == snk { return Some(f); }
    while wrk[cur] < adj[cur].len() {
        let nxt = adj[cur][wrk[cur]];
        if lvl[nxt] == lvl[cur] + 1 && cap[cur][nxt] - flo[cur][nxt] > 0 {
            if let Some(df) = dinic_dfs(flo, wrk, adj, cap, lvl, nxt, snk, f.min(cap[cur][nxt] - flo[cur][nxt])) {
                flo[cur][nxt] += df;
                flo[nxt][cur] -= df;
                return Some(df);
            }
        }
        wrk[cur] += 1;
    }
    None
}
fn dinic(flo: &mut Vec<Vec<i32>>, lvl: &mut Vec<i32>, wrk: &mut Vec<usize>,
         adj: &Vec<Vec<usize>>, cap: &Vec<Vec<i32>>, src: usize, snk: usize) -> i32 {
    let mut ret = 0;
    while dinic_bfs(lvl, adj, cap, flo, src, snk) {
        wrk.iter_mut().for_each(|x| *x = 0);
        while let Some(f) = dinic_dfs(flo, wrk, adj, cap, lvl, src, snk, INF) {
            ret += f;
        }
    }
    ret
}

pub fn main() { read();
    let (n, k, h, m) = (next::<usize>(), next::<i32>(), next::<usize>(), next::<usize>());

    let mut corners = vec![];
    for _ in 0..n { corners.push(Point { x: next(), y: next() }); }
    let mut lines = vec![];
    for i in 1..n { lines.push(Line { a: corners[i-1], b: corners[i] }); }
    lines.push(Line { a: corners[n-1], b: corners[0] });

    let mut holes = vec![];
    for _ in 0..h {
        let hole = Point { x: next(), y: next() };
        holes.push((hole, corners.contains(&hole)));
    }

    let mut mice = vec![];
    for _ in 0..m { mice.push(Point { x: next(), y: next() }); }

    let (src, snk) = (0, h+m+1);
    let mut adj = vec![vec![]; h+m+2];
    let mut cap = vec![vec![0; h+m+2]; h+m+2];
    let mut flo = vec![vec![0; h+m+2]; h+m+2];
    let mut lvl = vec![0; h+m+2];
    let mut wrk = vec![0; h+m+2];

    // connect source and mice
    for i in 1..=m {
        adj[src].push(i);
        adj[i].push(src);
        cap[src][i] = 1;
    }

    // connect mice and holes
    for i in 1..=m {
        for h in 1..=h {
            let (mouse, hole, corner) = (mice[i-1], holes[h-1].0, holes[h-1].1);
            let mh = Line { a: mouse, b: hole };
            let mut cnt = 0;
            for line in &lines {
                if intersect(&mh, line) { cnt += 1; }
                if cnt > 2 { break; }
            }
            if (corner && cnt == 2) || (!corner && cnt == 1) {
                adj[i].push(h+m);
                adj[h+m].push(i);
                cap[i][h+m] = k;
            }
        }
    }

    // connect holes and sink
    for h in 1..=h {
        adj[h+m].push(snk);
        adj[snk].push(h+m);
        cap[h+m][snk] = k;
    }

    let flow = dinic(&mut flo, &mut lvl, &mut wrk, &adj, &cap, src, snk);
    println!("{}", if flow == m as i32 { "Possible" } else { "Impossible" });
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}