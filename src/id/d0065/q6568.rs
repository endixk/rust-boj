// BOJ 6568 [Simple Computers]
#[derive(Default)] struct Computer {
    mm: [u8; 32],
    pc: u8,
    ac: u8,
    ha: bool,
}
impl Computer {
    fn sta(&mut self, ad: u8) { self.mm[ad as usize] = self.ac; }
    fn lda(&mut self, ad: u8) { self.ac = self.mm[ad as usize]; }
    fn beq(&mut self, ad: u8) { if self.ac == 0 { self.pc = ad; } }
    fn nop(&mut self, _ : u8) {}
    fn dec(&mut self, _ : u8) { self.ac = self.ac.wrapping_sub(1); }
    fn inc(&mut self, _ : u8) { self.ac = self.ac.wrapping_add(1); }
    fn jmp(&mut self, ad: u8) { self.pc = ad; }
    fn hlt(&mut self, _ : u8) { self.ha = true; }
    fn go(&mut self) {
        let op = self.mm[self.pc as usize] >> 5;
        let ad = self.mm[self.pc as usize] & 0x1f;
        self.pc = if self.pc == 31 { 0 } else { self.pc + 1 };
        match op {
            0 => self.sta(ad),
            1 => self.lda(ad),
            2 => self.beq(ad),
            3 => self.nop(ad),
            4 => self.dec(ad),
            5 => self.inc(ad),
            6 => self.jmp(ad),
            7 => self.hlt(ad),
            _ => unreachable!(),
        }
    }
}
pub fn main() { read();
    while peek() {
        let mut cpu = Computer::default();
        for i in 0..32 { cpu.mm[i] = u8::from_str_radix(&next::<String>(), 2).unwrap(); }
        while !cpu.ha { cpu.go(); }
        println!("{:08b}", cpu.ac);
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<std::iter::Peekable<SplitAsciiWhitespace>> = None;
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
fn peek() -> bool {
    unsafe { IT.as_mut().unwrap().peek().is_some() }
}