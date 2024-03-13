// BOJ 31431 [Python Code Folding]
struct SegTree<T> {
    n: usize,
    v: Vec<T>,
}
impl<T> SegTree<T> where
    T: std::ops::AddAssign + std::ops::Add<Output=T> + Default + Copy {
    fn new(n: usize) -> Self {
        Self { n: n.next_power_of_two(), v: vec![T::default(); n.next_power_of_two()<<1] }
    }
    fn update(&mut self, mut i: usize, x: T) {
        i |= self.n;
        self.v[i] = x;
        while i > 1 {
            i >>= 1;
            self.v[i] = self.v[i<<1] + self.v[i<<1|1];
        }
    }
    fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l |= self.n; r |= self.n;
        let mut ans = T::default();
        while l <= r {
            if l & 1 == 1 { ans += self.v[l]; l += 1; }
            if r & 1 == 0 { ans += self.v[r]; r -= 1; }
            l >>= 1; r >>= 1;
        }
        ans
    }
}
fn ett(adj: &Vec<Vec<usize>>, cur: usize, sz: &mut [usize]) -> usize {
    sz[cur] = 1;
    for &nxt in adj[cur].iter() {
        sz[cur] += ett(adj, nxt, sz);
    }
    sz[cur]
}
pub fn main() { read();
    let (n, q) = (next::<usize>(), next::<usize>());

    let mut stk = vec![0usize];
    let mut adj = vec![vec![]; n];
    let mut cnt = vec![1; n];
    let (mut it, mut ph) = (1, 0);
    for _ in 0..n {
        let (h, b) = (next::<usize>(), next::<char>() == 'h');
        for _ in h..ph {
            let x = stk.pop().unwrap();
            cnt[*stk.last().unwrap()] += cnt[x];
        }
        if b {
            adj[*stk.last().unwrap()].push(it);
            stk.push(it);
            it += 1;
        } else {
            cnt[*stk.last().unwrap()] += 1;
        }
        ph = h;
    }
    for _ in 0..ph {
        let x = stk.pop().unwrap();
        cnt[*stk.last().unwrap()] += cnt[x];
    }

    let mut sz = vec![0; n];
    ett(&adj, 0, &mut sz);
    let mut seg = SegTree::<usize>::new(n);
    for _ in 0..q {
        let p = next::<char>() == 'p';
        if p { println!("{}", cnt[0] - seg.v[1] - 1); }
        else {
            let i = next::<usize>();
            if seg.query(i, i) == 0 {
                let x = seg.query(i+1, i+sz[i]-1);
                seg.update(i, cnt[i] - x - 1);
            } else {
                seg.update(i, 0);
            }
        }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}