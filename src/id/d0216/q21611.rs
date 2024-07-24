// BOJ 21611 [Magician Shark and Blizzards]
fn _debug(arr: &Vec<u8>, n: usize) {
    let mut grid = vec![vec![0; n]; n];
    let (mut i, mut j) = (n>>1, n>>1);
    let mut k = 0;
    for l in 1..=n>>1 {
        j -= 1; i += 1;
        for _ in 0..l<<1 { i -= 1; grid[i][j] = arr[k]; k += 1; }
        for _ in 0..l<<1 { j += 1; grid[i][j] = arr[k]; k += 1; }
        for _ in 0..l<<1 { i += 1; grid[i][j] = arr[k]; k += 1; }
        for _ in 0..l<<1 { j -= 1; grid[i][j] = arr[k]; k += 1; }
    }
    for i in (0..n).rev() { for j in 0..n { print!("{:2} ", grid[i][j]); } println!(); }
    println!();
}
fn drop(arr: &mut Vec<u8>, i: usize) {
    if arr[i] == 0 { return; }
    let mut k = 0;
    for j in (0..i).rev() {
        if arr[j] != 0 { k = j+1; break; }
    }
    arr.swap(i, k);
}
fn pull(arr: &mut Vec<u8>, n: usize) {
    (0..n).for_each(|i| drop(arr, i));
}
fn explode(arr: &mut Vec<u8>, n: usize, a: &mut usize, b: &mut usize, c: &mut usize) -> bool {
    let mut flag = false;
    let (mut i, mut k) = (0, arr[0]);
    for j in 1..n {
        if arr[j] == k { continue; }
        if j-i >= 4 {
            match arr[i] { 1 => *a += j-i, 2 => *b += j-i, 3 => *c += j-i, _ => {} }
            flag |= arr[i] != 0;
            (i..j).for_each(|k| arr[k] = 0);
        }
        (i, k) = (j, arr[j]);
    }
    if n-i >= 4 {
        match arr[i] { 1 => *a += n-i, 2 => *b += n-i, 3 => *c += n-i, _ => {} }
        flag |= arr[i] != 0;
        (i..n).for_each(|k| arr[k] = 0);
    }
    pull(arr, n);
    flag
}
fn trans(arr: &mut Vec<u8>, n: usize) {
    let mut new = vec![];
    let (mut i, mut k) = (0, arr[0]);
    for j in 1..n {
        if arr[j] == k { continue; }
        new.push((j-i) as u8);
        new.push(k);
        (i, k) = (j, arr[j]);
    }
    if k != 0 {
        new.push((n-i) as u8);
        new.push(k);
    }
    new.resize(n, 0);
    arr.copy_from_slice(&new);
}
fn blizzard(arr: &mut Vec<u8>, n: usize, d: u8, s: u8) {
    let mut i = match d { 1 => 6, 2 => 2, 3 => 0, 4 => 4, _ => 0 };
    let mut k = match d { 1 => 15, 2 => 11, 3 => 9, 4 => 13, _ => 0 };
    for _ in 0..s {
        arr[i] = 0;
        i += k;
        k += 8;
    }
    pull(arr, n);
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut grid = vec![vec![0; n]; n];
    for i in (0..n).rev() { for j in 0..n { grid[i][j] = next(); } }

    let mut arr = vec![];
    let (mut i, mut j) = (n>>1, n>>1);
    for k in 1..=n>>1 {
        j -= 1; i += 1;
        for _ in 0..k<<1 { i -= 1; arr.push(grid[i][j]); }
        for _ in 0..k<<1 { j += 1; arr.push(grid[i][j]); }
        for _ in 0..k<<1 { i += 1; arr.push(grid[i][j]); }
        for _ in 0..k<<1 { j -= 1; arr.push(grid[i][j]); }
    }

    let l = arr.len();
    let (mut a, mut b, mut c) = (0, 0, 0);
    for _ in 0..m {
        let (d, s) = (next(), next());
        blizzard(&mut arr, l, d, s);
        while explode(&mut arr, l, &mut a, &mut b, &mut c) {}
        trans(&mut arr, l);
        // _debug(&arr, n);
    }
    println!("{}", a+b*2+c*3);
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