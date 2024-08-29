// BOJ 31467 [Chocolate Thief Coco]
fn artp(u: usize, init: bool, adj: &Vec<Vec<usize>>,
        reach: &mut Vec<usize>, it: &mut usize, art: &mut Vec<bool>) -> usize {
    *it += 1;
    reach[u] = *it;
    let mut nc = 0;
    let mut ret = reach[u];
    for &v in &adj[u] {
        if reach[v] != 0 {
            ret = ret.min(reach[v]);
            continue;
        }
        nc += 1;
        let r = artp(v, false, adj, reach, it, art);
        if !init && r >= reach[u] {
            art[u] = true;
        }
        ret = ret.min(r);
    }
    if init && nc > 1 { art[u] = true; }
    ret
}
pub fn main() { read();
    let n = next::<usize>();
    let mut b = vec![vec!['.'; n+2]; n+2];
    for i in 0..n {
        for (j, c) in next::<String>().chars().enumerate() {
            b[i+1][j+1] = c;
        }
    }

    let mut adj = vec![Vec::<usize>::with_capacity(4); 1<<20];
    let (mut v, mut e) = (0, 0);
    for i in 1..=n { for j in 1..=n {
        if b[i][j] == '.' { continue; }
        v += 1;
        if b[i-1][j] == '#' { e += 1; adj[i<<10|j].push((i-1)<<10|j); }
        if b[i+1][j] == '#' { e += 1; adj[i<<10|j].push((i+1)<<10|j); }
        if b[i][j-1] == '#' { e += 1; adj[i<<10|j].push(i<<10|j-1); }
        if b[i][j+1] == '#' { e += 1; adj[i<<10|j].push(i<<10|j+1); }
    }}
    e >>= 1;

    let mut reach = vec![0; n<<10|n+1];
    let mut it = 0;
    let mut art = vec![false; n<<10|n+1];
    for i in 1..=n { for j in 1..=n {
        if reach[i<<10|j] != 0 { continue; }
        artp(i<<10|j, true, &adj, &mut reach, &mut it, &mut art);
    }}

    let mut ans = vec![];
    for i in 1..=n { for j in 1..=n {
        if b[i][j] == '#' && !art[i<<10|j] {
            if e - adj[i<<10|j].len() == v - 2 { ans.push((i, j)); }
        }
    }}

    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i, j);
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}