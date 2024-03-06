// BOJ 27924 [Yunee Stole the Precious Thing]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
const SZ: usize = 200_000;
pub fn main() {
    let mut p = input(1 << 22);
    let n = ptr(&mut p) as usize;
    let mut adj = vec![vec![]; n];
    for _ in 1..n {
        let (u, v) = (ptr(&mut p) as usize - 1, ptr(&mut p) as usize - 1);
        adj[u].push(v); adj[v].push(u);
    }

    let (a, b, c) = (ptr(&mut p) as usize - 1, ptr(&mut p) as usize - 1, ptr(&mut p) as usize - 1);
    let mut log = [false; SZ];
    if adj[a].len() == 1 { println!("YES"); return; }
    else { log[a] = true; }
    log[b] = true; log[c] = true;

    use std::collections::VecDeque;
    let (mut qa, mut qb) = (VecDeque::from([a]), VecDeque::from([b, c]));
    while !qa.is_empty() || !qb.is_empty() {
        for _ in 0..qb.len() {
            let u = qb.pop_front().unwrap();
            for &v in &adj[u] {
                if !log[v] {
                    log[v] = true;
                    if adj[v].len() != 1 { qb.push_back(v); }
                }
            }
        }
        for _ in 0..qa.len() {
            let u = qa.pop_front().unwrap();
            for &v in &adj[u] {
                if !log[v] {
                    log[v] = true;
                    if adj[v].len() == 1 { println!("YES"); return; }
                    else { qa.push_back(v); }
                }
            }
        }
    }
    println!("NO");
}