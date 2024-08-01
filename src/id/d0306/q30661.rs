// BOJ 30661 [Archipelago]
fn vc(b: &[char; 64], i: usize, j: usize, n: usize, m: usize) -> char {
    ('0' as u8
        + (i > 0 && b[i-1<<3|j] == 'N') as u8
        + (j > 0 && b[i<<3|j-1] == 'N') as u8
        + (i < n-1 && b[i+1<<3|j] == 'N') as u8
        + (j < m-1 && b[i<<3|j+1] == 'N') as u8) as char
}
fn valid(b: &[char; 64], n: usize, m: usize) -> bool {
    for i in 0..n { for j in 0..m {
        if b[i<<3|j] == '.' { return false; }
        if b[i<<3|j].is_digit(10) && b[i<<3|j] != '0' { return false; }
    }}
    true
}
fn go(b: &mut [char; 64], n: usize, m: usize, i: usize, j: usize) -> bool {
    if i == n {
        if valid(b, n, m) {
            for i in 0..n {
                for j in 0..m {
                    print!("{}", if b[i<<3|j] == 'O' { '.' } else if b[i<<3|j] == '0' { vc(b, i, j, n, m) } else { b[i<<3|j] });
                }
                println!();
            }
            return true;
        }
        return false;
    }

    let (mut ni, mut nj) = (i, j+1);
    if nj == m { ni += 1; nj = 0; }
    if b[i<<3|j] != '.' { return go(b, n, m, ni, nj); }

    // place boat
    b[i<<3|j] = 'N';
    let mut v = vec![];
    let f = (i > 0 && b[i-1<<3|j] == '0') || (i < n-1 && b[i+1<<3|j] == '0') || (j > 0 && b[i<<3|j-1] == '0') || (j < m-1 && b[i<<3|j+1] == '0');
    if !f {
        if i > 0 && b[i-1<<3|j].is_digit(10) {
            b[i-1<<3|j] = match b[i-1<<3|j] {
                '1' => '0', '2' => '1', '3' => '2', _ => '3',
            }
        }
        if i < n-1 && b[i+1<<3|j].is_digit(10) {
            b[i+1<<3|j] = match b[i+1<<3|j] {
                '1' => '0', '2' => '1', '3' => '2', _ => '3',
            }
        }
        if j > 0 && b[i<<3|j-1].is_digit(10) {
            b[i<<3|j-1] = match b[i<<3|j-1] {
                '1' => '0', '2' => '1', '3' => '2', _ => '3',
            }
        }
        if j < m-1 && b[i<<3|j+1].is_digit(10) {
            b[i<<3|j+1] = match b[i<<3|j+1] {
                '1' => '0', '2' => '1', '3' => '2', _ => '3',
            }
        }
        for k in (0..i).rev() {
            if b[k<<3|j] != '.' && b[k<<3|j] != 'O' { break; }
            if b[k<<3|j] == '.' { b[k<<3|j] = 'O'; v.push((k, j)) }
        }
        for k in i+1..n {
            if b[k<<3|j] != '.' && b[k<<3|j] != 'O' { break; }
            if b[k<<3|j] == '.' { b[k<<3|j] = 'O'; v.push((k, j)) }
        }
        for k in (0..j).rev() {
            if b[i<<3|k] != '.' && b[i<<3|k] != 'O' { break; }
            if b[i<<3|k] == '.' { b[i<<3|k] = 'O'; v.push((i, k)) }
        }
        for k in j+1..m {
            if b[i<<3|k] != '.' && b[i<<3|k] != 'O' { break; }
            if b[i<<3|k] == '.' { b[i<<3|k] = 'O'; v.push((i, k)) }
        }
        if go(b, n, m, ni, nj) { return true; }
    }

    // remove boat
    b[i<<3|j] = '.';
    if !f {
        for (i, j) in v { b[i<<3|j] = '.'; }
        if i > 0 && b[i-1<<3|j].is_digit(10) {
            b[i-1<<3|j] = match b[i-1<<3|j] {
                '0' => '1', '1' => '2', '2' => '3', _ => '4',
            }
        }
        if i < n-1 && b[i+1<<3|j].is_digit(10) {
            b[i+1<<3|j] = match b[i+1<<3|j] {
                '0' => '1', '1' => '2', '2' => '3', _ => '4',
            }
        }
        if j > 0 && b[i<<3|j-1].is_digit(10) {
            b[i<<3|j-1] = match b[i<<3|j-1] {
                '0' => '1', '1' => '2', '2' => '3', _ => '4',
            }
        }
        if j < m-1 && b[i<<3|j+1].is_digit(10) {
            b[i<<3|j+1] = match b[i<<3|j+1] {
                '0' => '1', '1' => '2', '2' => '3', _ => '4',
            }
        }
    }
    go(b, n, m, ni, nj)
}
pub fn main() { read();
    loop {
        let (n, m) = (next::<usize>(), next::<usize>());
        if n == 0 && m == 0 { break; }
        let mut b = ['X'; 64];
        for i in 0..n {
            let s = next::<String>();
            for (j, c) in s.chars().enumerate() {
                b[i<<3|j] = c;
            }
        }
        if !go(&mut b, n, m, 0, 0) { println!("IMPOSSIVEL"); }
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