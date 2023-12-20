fn bip(adj: &Vec<Vec<usize>>, mat: &mut Vec<usize>, vis: &mut Vec<bool>, cur: usize) -> bool {
    for &nxt in &adj[cur] {
        if vis[nxt] { continue; }
        vis[nxt] = true;
        if mat[nxt] == 0 || bip(adj, mat, vis, mat[nxt]) {
            mat[nxt] = cur;
            return true;
        }
    }
    false
}
pub fn main() { read();
    // adjacency list of a 16-cell
    let mut adj = vec![vec![],
        vec![2, 3, 4, 5], vec![1, 6, 7, 8], vec![1, 6, 9, 10], vec![1, 7, 9, 11],
        vec![1, 8, 10, 11], vec![2, 3, 12, 13], vec![2, 4, 12, 14], vec![2, 5, 13, 14],
        vec![3, 4, 12, 15], vec![3, 5, 13, 15], vec![4, 5, 14, 15], vec![6, 7, 9, 16],
        vec![6, 8, 10, 16], vec![7, 8, 11, 16], vec![9, 10, 11, 16], vec![12, 13, 14, 15]
    ];
    for _ in 0..next() {
        let (i, j) = (next::<usize>(), next::<usize>());
        let ij = adj[i].iter().position(|&x| x == j).unwrap();
        let ji = adj[j].iter().position(|&x| x == i).unwrap();
        adj[i].swap_remove(ij);
        adj[j].swap_remove(ji);
    }

    let mut mat = vec![0; 17];
    let mut ans = 0;
    for i in vec![1, 6, 7, 8, 9, 10, 11, 16] {
        let mut vis = vec![false; 17];
        if bip(&adj, &mut mat, &mut vis, i) { ans += 1; }
    }

    let mut lit = vec![false; 17];
    for i in 1..17 {
        if mat[i] == 0 { continue; }
        lit[i] = true; lit[mat[i]] = true;
    }
    ans += lit.iter().filter(|&&x| !x).count() - 1;
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