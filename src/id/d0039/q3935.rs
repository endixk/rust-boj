// BOJ 3935 [Weather Forecast]
const BIT: [u16; 9] = [
    0b1100110000000000, 0b0110011000000000, 0b0011001100000000,
    0b0000110011000000, 0b0000011001100000, 0b0000001100110000,
    0b0000000011001100, 0b0000000001100110, 0b0000000000110011,
];
const ADJ: [[usize; 5]; 9] = [
    [0, 1, 2, 3, 6], [1, 0, 2, 4, 7], [2, 0, 1, 5, 8],
    [3, 0, 4, 5, 6], [4, 1, 3, 5, 7], [5, 2, 3, 4, 8],
    [6, 0, 3, 7, 8], [7, 1, 4, 6, 8], [8, 2, 5, 6, 7],
];
fn decode(bit: usize, dec: &Vec<(u8,u8,u8,u8,u8,u8)>) -> (usize, usize, usize, usize, usize, usize, usize) {
    let mut bit = bit;
    let ini = bit & 0b1111; bit >>= 4;
    let (m1, m2, m3, m4, m5, m6) = dec[bit];
    (ini, m1 as usize, m2 as usize, m3 as usize, m4 as usize, m5 as usize, m6 as usize)
}
fn encode(ini: usize, m1: usize, m2: usize, m3: usize, m4: usize, m5: usize, m6: usize, enc: &[[[[[[usize;5];5];5];5];5];5]) -> usize {
    ini | enc[m1][m2][m3][m4][m5][m6] << 4
}
fn get(bit: usize, dec: &Vec<(u8,u8,u8,u8,u8,u8)>, enc: &[[[[[[usize;5];5];5];5];5];5]) -> [usize; 5] {
    let (ini, m1, m2, m3, m4, m5, m6) = decode(bit, dec);
    let nxt = ADJ[ini][m1];
    [
        encode(nxt, m2, m3, m4, m5, m6, 0, enc),
        encode(nxt, m2, m3, m4, m5, m6, 1, enc),
        encode(nxt, m2, m3, m4, m5, m6, 2, enc),
        encode(nxt, m2, m3, m4, m5, m6, 3, enc),
        encode(nxt, m2, m3, m4, m5, m6, 4, enc),
    ]
}
fn valid(bit: usize, day: usize, states: &Vec<u16>, dec: &Vec<(u8,u8,u8,u8,u8,u8)>) -> bool {
    let (ini, m1, m2, m3, m4, m5, m6) = decode(bit, dec);
    let s0 = ini;
    let s1 = ADJ[s0][m1];
    let s2 = ADJ[s1][m2];
    let s3 = ADJ[s2][m3];
    let s4 = ADJ[s3][m4];
    let s5 = ADJ[s4][m5];
    let s6 = ADJ[s5][m6];
    if BIT[s0] | BIT[s1] | BIT[s2] | BIT[s3] | BIT[s4] | BIT[s5] | BIT[s6] != u16::MAX { return false; }
    if states[day] & BIT[s0] != 0 { return false; }
    if states[day + 1] & BIT[s1] != 0 { return false; }
    if states[day + 2] & BIT[s2] != 0 { return false; }
    if states[day + 3] & BIT[s3] != 0 { return false; }
    if states[day + 4] & BIT[s4] != 0 { return false; }
    if states[day + 5] & BIT[s5] != 0 { return false; }
    if states[day + 6] & BIT[s6] != 0 { return false; }
    true
}
pub fn main() { read();
    let mut dec = vec![];
    let mut enc = [[[[[[0;5];5];5];5];5];5];
    let mut it = 0;
    for m1 in 0..5 { for m2 in 0..5 { for m3 in 0..5 { for m4 in 0..5 { for m5 in 0..5 { for m6 in 0..5 {
        dec.push((m1 as u8, m2 as u8, m3 as u8, m4 as u8, m5 as u8, m6 as u8));
        enc[m1][m2][m3][m4][m5][m6] = it;
        it += 1;
    }}}}}}
    let mut vis = vec![false; it << 4];

    loop {
        let n = next::<usize>();
        if n == 0 { break; }
        let mut states = vec![0; if n < 7 { 7 } else { n }];
        for i in 0..n {
            let mut state = 0;
            for _ in 0..16 {
                state <<= 1;
                state |= next::<u16>();
            }
            states[i] = state;
        }

        let n = states.len();
        let mut v = vec![];
        for bit in 0..it {
            let bit = 4 | (bit << 4);
            if valid(bit, 0, &states, &dec) { v.push(bit); }
        }

        for day in 1..=n-7 {
            vis.fill(false);
            let mut nv = vec![];
            for &bit in &v {
                for &nxt in &get(bit, &dec, &enc) {
                    if !vis[nxt] && valid(nxt, day, &states, &dec) {
                        vis[nxt] = true;
                        nv.push(nxt);
                    }
                }
            }
            v = nv;
            if v.is_empty() { break; }
        }

        println!("{}", if v.is_empty() { 0 } else { 1 });
    }
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