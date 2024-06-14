// BOJ 15265 [Hidden Hierarchy]
use std::collections::HashMap;

fn proc(tree: &Vec<Vec<usize>>, sz: &Vec<usize>, vec: &Vec<String>, thres: usize, cur: usize) {
    if tree[cur].is_empty() {
        println!("  {} {}", vec[cur], sz[cur]);
        return;
    }
    if tree[cur].iter().all(|&x| sz[x] < thres) {
        println!("+ {} {}", vec[cur], sz[cur]);
        return;
    }

    println!("- {} {}", vec[cur], sz[cur]);
    let mut ord = tree[cur].clone();
    ord.sort_by_key(|&x| vec[x].clone());
    for &x in ord.iter() {
        proc(tree, sz, vec, thres, x);
    }
}

pub fn main() { read();
    let mut map = HashMap::<String, usize>::new();
    let mut vec = vec![];
    map.insert("/".to_string(), 0);
    vec.push("/".to_string());

    let mut tree = vec![vec![]];
    let mut idx = 1;
    let mut sz = vec![0];

    for _ in 0..next() {
        let (path, size) = (next::<String>(), next::<usize>());
        let mut sub = "/".to_string();
        sz[0] += size;
        for dir in path[..path.rfind('/').unwrap()].split('/').skip(1) {
            let par = map[&sub];
            sub.push_str(dir);
            sub.push('/');
            if !map.contains_key(&sub) {
                map.insert(sub.clone(), idx);
                vec.push(sub.clone());
                sz.push(0);
                tree[par].push(idx);
                tree.push(vec![]);
                idx += 1;
            }
            sz[map[&sub]] += size;
        }
    }

    let thres = next::<usize>();
    proc(&tree, &sz, &vec, thres, 0);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
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