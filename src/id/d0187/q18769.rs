// BOJ 18769 [Grid Network]
fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x { x }
    else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let (x, y) = (find(root, x), find(root, y));
    if x == y { return; }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}
pub fn main() { read();
    for _ in 0..next() {
        let (r, c) = (next::<usize>(), next::<usize>());
        let (mut e1, mut e2, mut e3, mut e4) = (vec![], vec![], vec![], vec![]);
        for i in 0..r {
            for j in 1..c {
                match next::<u8>() {
                    1 => e1.push((1, i*c+j-1, i*c+j)),
                    2 => e2.push((2, i*c+j-1, i*c+j)),
                    3 => e3.push((3, i*c+j-1, i*c+j)),
                    _ => e4.push((4, i*c+j-1, i*c+j)),
                }
            }
        }
        for i in 1..r {
            for j in 0..c {
                match next::<u8>() {
                    1 => e1.push((1, (i-1)*c+j, i*c+j)),
                    2 => e2.push((2, (i-1)*c+j, i*c+j)),
                    3 => e3.push((3, (i-1)*c+j, i*c+j)),
                    _ => e4.push((4, (i-1)*c+j, i*c+j)),
                }
            }
        }

        let mut edges = vec![];
        edges.extend(e1.into_iter());
        edges.extend(e2.into_iter());
        edges.extend(e3.into_iter());
        edges.extend(e4.into_iter());

        let mut root = (0..r*c).collect::<Vec<usize>>();
        let mut rank = vec![0; r*c];
        let (mut cnt, mut mst) = (0, 0);
        for (w, u, v) in edges {
            if find(&mut root, u) != find(&mut root, v) {
                union(&mut root, &mut rank, u, v);
                cnt += 1;
                mst += w;
            }
            if cnt == r*c-1 { break; }
        }

        println!("{}", mst);
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