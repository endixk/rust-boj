// BOJ 6579 [Friends]
pub fn main() { read();
    while peek() {
        let mut pf = vec![];
        let mut st = vec![];
        let cmd = next::<String>().into_bytes();
        let mut i = 0;
        while i < cmd.len() {
            match cmd[i] {
                b'{' => {
                    let mut x = 0;
                    i += 1;
                    while cmd[i] != b'}' {
                        x |= 1 << cmd[i] - b'A';
                        i += 1;
                    }
                    pf.push(x);
                },
                b'(' => st.push(b'('),
                b')' => {
                    loop {
                        let c = st.pop().unwrap();
                        if c == b'(' { break; }
                        match c {
                            b'+' => pf.push(-1),
                            b'-' => pf.push(-2),
                            b'*' => pf.push(-3),
                            _ => (),
                        }
                    }
                }
                b'+' => {
                    while let Some(&c) = st.last() {
                        if c == b'(' { break; }
                        match c {
                            b'+' => pf.push(-1),
                            b'-' => pf.push(-2),
                            b'*' => pf.push(-3),
                            _ => (),
                        }
                        st.pop();
                    }
                    st.push(b'+');
                }
                b'-' => {
                    while let Some(&c) = st.last() {
                        if c == b'(' { break; }
                        match c {
                            b'+' => pf.push(-1),
                            b'-' => pf.push(-2),
                            b'*' => pf.push(-3),
                            _ => (),
                        }
                        st.pop();
                    }
                    st.push(b'-');
                }
                b'*' => {
                    while let Some(&c) = st.last() {
                        if c == b'(' || c == b'+' || c == b'-' { break; }
                        match c {
                            b'*' => pf.push(-3),
                            _ => (),
                        }
                        st.pop();
                    }
                    st.push(b'*');
                }
                _ => (),
            }
            i += 1;
        }
        while let Some(c) = st.pop() {
            match c {
                b'+' => pf.push(-1),
                b'-' => pf.push(-2),
                b'*' => pf.push(-3),
                _ => (),
            }
        }

        let mut st = vec![];
        for &x in &pf {
            if x >= 0 {
                st.push(x);
            } else {
                let b = st.pop().unwrap();
                let a = st.pop().unwrap();
                match x {
                    -1 => st.push(a | b),
                    -2 => st.push(a & !b),
                    -3 => st.push(a & b),
                    _ => (),
                }
            }
        }

        print!("{{");
        for i in 0..26 {
            if st[0] & 1 << i != 0 {
                print!("{}", (b'A' + i as u8) as char);
            }
        }
        println!("}}");
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }
