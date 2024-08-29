// BOJ 14216 [Assignment 2]
// Reference: https://www.topcoder.com/thrive/articles/Assignment_Problem_and_Hungarian_Algorithm
use std::collections::VecDeque;
const MAX: i32 = 10000;
const INF: i32 = 0x3f3f3f3f;
fn update(xlab: &mut Vec<i32>, ylab: &mut Vec<i32>, slk: &mut Vec<i32>,
                s: &[bool], t: &[bool], n: usize) {
    let mut d = INF;
    for i in 0..n {
        if !t[i] { d = d.min(slk[i]); }
    }
    for i in 0..n {
        if s[i] { xlab[i] -= d; }
        if t[i] { ylab[i] += d; }
        else { slk[i] -= d; }
    }
}
fn augment(i: usize, p: usize,
           s: &mut Vec<bool>, prv: &mut Vec<usize>, slk: &mut Vec<i32>, slx: &mut Vec<usize>,
           xlab: &[i32], ylab: &[i32], cost: &Vec<Vec<i32>>, n: usize) {
    s[i] = true; prv[i] = p;
    for j in 0..n {
        if xlab[i] + ylab[j] - cost[i][j] < slk[j] {
            slk[j] = xlab[i] + ylab[j] - cost[i][j];
            slx[j] = i;
        }
    }
}
fn hungarian(cost: &Vec<Vec<i32>>, n: usize) -> i32 {
    // initialize variables
    let mut xlab = (0..n).map(|i| *cost[i].iter().max().unwrap()).collect::<Vec<_>>();
    let mut ylab = vec![0; n];
    let mut xmat = vec![n; n];
    let mut ymat = vec![n; n];

    // augmentation loop
    for _ in 0..n {
        // initialize variables
        let mut root = 0;
        let mut prv = vec![n; n];
        let (mut s, mut t) = (vec![false; n], vec![false; n]);
        let mut q = VecDeque::new();

        // find root
        for i in 0..n {
            if xmat[i] == n {
                root = i; q.push_back(i);
                prv[i] = n+1; s[i] = true; break;
            }
        }
        // initialize slack array
        let mut slk = (0..n).map(|i| xlab[root] + ylab[i] - cost[root][i]).collect::<Vec<_>>();
        let mut slx = vec![root; n];

        // main loop
        let (mut ex, mut ey);
        'x: loop {
            // build alternating tree
            while let Some(i) = q.pop_front() {
                for j in 0..n {
                    if !t[j] && cost[i][j] == xlab[i] + ylab[j] {
                        if ymat[j] == n { (ex, ey) = (i, j); break 'x; }
                        t[j] = true; q.push_back(ymat[j]);
                        augment(ymat[j], i, &mut s, &mut prv, &mut slk, &mut slx, &xlab, &ylab, &cost, n);
                    }
                }
            }
            // update labels
            update(&mut xlab, &mut ylab, &mut slk, &s, &t, n);
            // update matching
            for j in 0..n {
                if !t[j] && slk[j] == 0 {
                    if ymat[j] == n { (ex, ey) = (slx[j], j); break 'x; }
                    t[j] = true;
                    if !s[ymat[j]] {
                        q.push_back(ymat[j]);
                        augment(ymat[j], slx[j], &mut s, &mut prv, &mut slk, &mut slx, &xlab, &ylab, &cost, n);
                    }
                }
            }
        }

        // invert the augmenting path
        while ex != n+1 {
            let t = xmat[ex];
            (xmat[ex], ymat[ey]) = (ey, ex);
            (ex, ey) = (prv[ex], t);
        }
    }

    // calculate the cost
    (0..n).map(|i| cost[i][xmat[i]]).sum()
}
pub fn main() { read();
    let n = next::<usize>();
    let mut cost = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n {
        cost[i][j] = MAX - next::<i32>();
    }}
    println!("{}", MAX * n as i32 - hungarian(&cost, n));
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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