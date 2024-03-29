// BOJ 28073 [PS Aaak Training]
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let (s, e) = (next::<char>() as u8, next::<char>() as u8);
    let mut c = vec![b'*'];
    c.extend(next::<String>().into_bytes());

    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (next::<usize>(), next::<usize>());
        adj[u].push(v);
        adj[v].push(u);
    }
    for i in 1..=n {
        if c[i] == s { adj[0].push(i); }
    }

    let mut q = std::collections::VecDeque::new();
    let mut p = vec![n+1; n+1];
    let mut r = 0;
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        if c[u] == e { r = u; break; }
        adj[u].sort_unstable_by(|a, b| c[*a].cmp(&c[*b]));
        adj[u].dedup();

        let mut alp = vec![0; 26];
        let nxt = adj[u].clone();
        for v in nxt {
            if p[v] == n+1 {
                p[v] = u;
                let i = (c[v] - b'A') as usize;
                if alp[i] != 0 {
                    let w = adj[v].clone();
                    adj[alp[i]].extend(w);
                } else {
                    alp[i] = v;
                    q.push_back(v);
                }
            }
        }
    }

    if r == 0 { println!("Aaak!"); return; }
    let mut ans = vec![];
    while r != 0 {
        ans.push(c[r] as char);
        r = p[r];
    }
    ans.reverse();
    println!("{}", ans.into_iter().collect::<String>());
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}