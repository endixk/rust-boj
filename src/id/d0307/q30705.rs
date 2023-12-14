// BOJ 30705 [ENDLESS RAIN]
// Supported by GitHub Copilot

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x { x }
    else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union_sz(root: &mut Vec<usize>,
            rank: &mut Vec<usize>,
            size: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            size[y] += size[x];
        } else {
            root[y] = x;
            size[x] += size[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
        }
    }
}

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());

    let mut tail = (0..=n).collect::<Vec<usize>>();
    let mut root = (0..=n).collect::<Vec<usize>>();
    let mut rank = vec![0; n+1];
    let mut size = vec![1; n+1];
    let mut vis = vec![false; n+1];

    let mut ans = 0;
    let mut cnt = 0;
    for _ in 0..m {
        cnt += 1;
        let (l, r) = (next::<usize>(), next::<usize>());
        let (mut i, mut k) = (l, 0);
        while i < r {
            if !vis[i] {
                vis[i] = true; k += 1;
                if i > 1 && vis[i-1] {
                    union_sz(&mut root, &mut rank, &mut size, i-1, i);
                    tail[find(&mut root, i)] = i;
                }
                if i < n && vis[i+1] {
                    let t = tail[find(&mut root, i+1)];
                    union_sz(&mut root, &mut rank, &mut size, i, i+1);
                    tail[find(&mut root, i)] = t;
                }
            }
            let r = find(&mut root, i);
            i = tail[r] + 1;
        }

        if cnt > k { cnt -= k; }
        else { ans += k - cnt; cnt = 0; }
    }

    println!("{}", ans);
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