// BOJ 6577 [From Dusk till Dawn]
use std::collections::{HashMap, BinaryHeap};
pub fn main() { read();
    for tc in 0..next() {
        let mut map: HashMap<String, usize> = HashMap::new();
        let mut adj = vec![vec![]; 100];
        let mut idx = 0;
        for _ in 0..next() {
            let (s, t, d, e) = (next::<String>(), next::<String>(), (next::<i32>() + 6) % 24, next::<i32>());
            let s = *map.entry(s).or_insert_with(|| {idx += 1; idx - 1});
            let t = *map.entry(t).or_insert_with(|| {idx += 1; idx - 1});
            adj[s].push((t, d, e));
        }

        let (src, dst) = (*map.get(&next::<String>()).unwrap(), *map.get(&next::<String>()).unwrap());
        let mut pq = BinaryHeap::new();
        let mut d = vec![0x3f3f3f3f; 100];
        d[src] = 0; pq.push((0, src));
        while let Some((dist, u)) = pq.pop() {
            let dist = -dist;
            if d[u] < dist { continue; }
            let (x, y) = (dist / 24, dist % 24);
            for &(v, w, e) in &adj[u] {
                if w + e > 12 { continue; }
                if w >= y {
                    if d[v] > x * 24 + w + e {
                        d[v] = x * 24 + w + e;
                        pq.push((-d[v], v));
                    }
                } else {
                    if d[v] > (x + 1) * 24 + w + e {
                        d[v] = (x + 1) * 24 + w + e;
                        pq.push((-d[v], v));
                    }
                }
            }
        }

        println!("Test Case {}.", tc + 1);
        if d[dst] == 0x3f3f3f3f { println!("There is no route Vladimir can take.") }
        else { println!("Vladimir needs {d} litre(s) of blood.", d = d[dst] / 24); }
    }
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