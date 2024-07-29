// BOJ 6328 [Erdos Numbers]
pub fn main() {
    for tc in 1.. {
        let s = next();
        let p = s.split_ascii_whitespace().next().unwrap().parse::<usize>().unwrap();
        if p == 0 { break; }
        let n = s.split_ascii_whitespace().skip(1).next().unwrap().parse::<usize>().unwrap();

        let mut map = std::collections::HashMap::new();
        map.insert("Erdos, P.".to_string(), 0);
        let mut it = 0;

        let mut adj = vec![vec![]];
        for _ in 0..p {
            let s = next();
            let s = s[0..s.find(':').unwrap()].to_string();
            let v = s.split(", ").map(|x| x.to_string()).collect::<Vec<_>>();

            let mut w = vec![];
            for i in (0..v.len()).step_by(2) {
                let q = v[i].clone() + ", " + &v[i+1];
                w.push(*map.entry(q).or_insert_with(|| { it += 1; adj.push(vec![]); it }));
            }

            for i in 0..w.len()-1 { for j in i+1..w.len() {
                adj[w[i]].push(w[j]);
                adj[w[j]].push(w[i]);
            }}
        }

        for v in adj.iter_mut() { v.sort(); v.dedup(); }
        let mut d = vec![-1; it+1];
        d[0] = 0;
        let mut q = vec![0];
        while !q.is_empty() {
            let mut nq = vec![];
            for &u in &q {
                for &v in &adj[u] {
                    if d[v] == -1 {
                        d[v] = d[u] + 1;
                        nq.push(v);
                    }
                }
            }
            q = nq;
        }

        println!("Database #{}", tc);
        for _ in 0..n {
            let w = next();
            if !map.contains_key(&w) { println!("{}: infinity", w); continue; }
            println!("{}: {}", w, if d[map[&w]] == -1 { "infinity".to_string() } else { d[map[&w]].to_string() });
        }
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
use println;
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
fn flush() { SO.with(|c| c.borrow_mut().flush().unwrap()); }