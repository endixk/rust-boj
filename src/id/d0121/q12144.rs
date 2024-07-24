// BOJ 12144 [Bilingual (Large)]
use std::collections::VecDeque;
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
pub fn main() {
    for tc in 0..next().parse().unwrap() {
        let n = next().parse::<usize>().unwrap();
        let v = (0..n).map(|_| next()).collect::<Vec<_>>();

        let mut map = std::collections::HashMap::new();
        let mut it = 0;
        for s in &v[2..] {
            for w in s.split_ascii_whitespace() {
                map.entry(w).or_insert_with(|| { it += 1; it - 1 });
            }
        }

        let sz = it * 2 + 2;
        let mut adj = vec![vec![]; sz];
        let mut cap = vec![vec![0; sz]; sz];
        let (src, snk) = (sz - 2, sz - 1);

        let mut ans = 0;
        let mut exc = std::collections::HashSet::new();
        for w in v[0].split_ascii_whitespace() {
            let x = if let Some(&x) = map.get(w) { x } else {
                exc.insert(w);
                continue
            };
            adj[src].push(x); adj[x].push(src);
            cap[src][x] = INF;
        }
        for w in v[1].split_ascii_whitespace() {
            let x = if let Some(&x) = map.get(w) { x + it } else {
                if exc.contains(w) { ans += 1; exc.remove(w); }
                continue
            };
            adj[x].push(snk); adj[snk].push(x);
            cap[x][snk] = INF;
        }
        for x in 0..it {
            adj[x].push(x + it); adj[x + it].push(x);
            cap[x][x + it] = 1;
        }

        for s in &v[2..] {
            let w = s.split_ascii_whitespace().collect::<Vec<_>>();
            for i in 0..w.len()-1 { for j in i+1..w.len() {
                let (x, y) = (map[w[i]], map[w[j]]);
                if x == y { continue; }
                adj[x].push(y + it); adj[y + it].push(x);
                adj[y].push(x + it); adj[x + it].push(y);
                cap[y + it][x] = INF;
                cap[x + it][y] = INF;
            }}
        }

        let mut flo = vec![vec![0; sz]; sz];
        let mut lvl = vec![0; sz];
        let mut wrk = vec![0; sz];
        println!("Case #{}: {}", tc + 1, ans + dinic(&mut flo, &mut lvl, &mut wrk, &adj, &cap, src, snk));
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn next() -> String { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_line(&mut *addr_of_mut!(BUF)).unwrap());
    BUF.trim_end().to_string()
}}