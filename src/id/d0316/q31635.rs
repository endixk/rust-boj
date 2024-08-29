// BOJ 31635 [Gaji and the Maze]
pub fn main() {
    let n = next().trim().parse::<usize>().unwrap();
    let mut adj = vec![vec![]; n+1];
    let mut p = vec![0; n+1];
    let (mut cur, mut cnt) = (1, 1);
    p[1] = 1;
    while cnt < n {
        println!("maze"); flush();
        let nxt = next().trim().parse::<usize>().unwrap();
        if p[nxt] == 0 {
            adj[cur].push(nxt);
            p[nxt] = cur;
            cur = nxt;
            cnt += 1;
        } else {
            if nxt == p[cur] { cur = nxt; }
            else {
                println!("gaji {}", cur); flush(); next();
                println!("gaji {}", p[cur]); flush(); next();
                cur = p[cur];
            }
        }
    }

    println!("answer"); flush();
    for u in 1..=n {
        for &v in &adj[u] {
            println!("{} {}", u, v); flush();
        }
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
    BUF.clone()
}}
fn flush() { SO.with(|c| c.borrow_mut().flush().unwrap()); }