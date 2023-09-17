use std::io::{self, Read, Write};
fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}
fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

use std::collections::VecDeque;
struct Queueueue {
    size: usize,
    c: i32,
    vqf: VecDeque<i32>,
    vqb: VecDeque<i32>,
    hqf: VecDeque<i32>,
    hqb: VecDeque<i32>,
}
impl Queueueue {
    fn new() -> Self {
        Queueueue {
            size: 0,
            c: 0,
            vqf: VecDeque::new(),
            vqb: VecDeque::new(),
            hqf: VecDeque::new(),
            hqb: VecDeque::new(),
        }
    }
    fn heq(&mut self) {
        if self.hqf.len() == self.hqb.len() + 2 {
            self.hqb.push_back(self.c);
            self.c = self.hqf.pop_front().unwrap();
        } else if self.hqb.len() == self.hqf.len() + 1 {
            self.hqf.push_front(self.c);
            self.c = self.hqb.pop_back().unwrap();
        }
    }
    fn veq(&mut self) {
        if self.vqf.len() == self.vqb.len() + 2 {
            self.vqb.push_back(self.c);
            self.c = self.vqf.pop_front().unwrap();
        } else if self.vqb.len() == self.vqf.len() + 1 {
            self.vqf.push_front(self.c);
            self.c = self.vqb.pop_back().unwrap();
        }
    }
    fn hpush(&mut self, x: i32) {
        if self.size == 0 {
            self.c = x;
            self.size += 1;
            return;
        }
        self.hqf.push_back(x);
        self.heq();
        self.size += 1;
    }
    fn hpop(&mut self) -> i32 {
        if self.size == 0 { return -1; }
        if self.hqf.is_empty() && self.hqb.is_empty() {
            let x = self.c;
            if !self.vqb.is_empty() {
                self.c = self.vqb.pop_back().unwrap();
                self.veq();
            } else if !self.vqf.is_empty() {
                self.c = self.vqf.pop_front().unwrap();
                self.veq();
            } else {
                self.c = 0;
            }
            self.size -= 1;
            return x;
        }
        if self.hqb.is_empty() {
            let x = self.c;
            self.c = self.hqf.pop_front().unwrap();
            self.heq();
            self.size -= 1;
            return x;
        }
        let x = self.hqb.pop_front().unwrap();
        self.heq();
        self.size -= 1;
        x
    }
    fn hfront(&self) -> i32 {
        if self.size == 0 { return -1; }
        if self.hqf.is_empty() && self.hqb.is_empty() {
            return self.c;
        }
        if self.hqb.is_empty() {
            return self.c;
        }
        *self.hqb.front().unwrap()
    }
    fn hback(&self) -> i32 {
        if self.size == 0 { return -1; }
        if self.hqf.is_empty() && self.hqb.is_empty() {
            return self.c;
        }
        if self.hqf.is_empty() {
            return self.c;
        }
        *self.hqf.back().unwrap()
    }
    fn hsize(&self) -> usize {
        self.hqf.len() + self.hqb.len() + if self.size == 0 { 0 } else { 1 }
    }
    fn vpush(&mut self, x: i32) {
        if self.size == 0 {
            self.c = x;
            self.size += 1;
            return;
        }
        self.vqf.push_back(x);
        self.veq();
        self.size += 1;
    }
    fn vpop(&mut self) -> i32 {
        if self.size == 0 { return -1; }
        if self.vqf.is_empty() && self.vqb.is_empty() {
            let x = self.c;
            if !self.hqb.is_empty() {
                self.c = self.hqb.pop_back().unwrap();
                self.heq();
            } else if !self.hqf.is_empty() {
                self.c = self.hqf.pop_front().unwrap();
                self.heq();
            } else {
                self.c = 0;
            }
            self.size -= 1;
            return x;
        }
        if self.vqb.is_empty() {
            let x = self.c;
            self.c = self.vqf.pop_front().unwrap();
            self.veq();
            self.size -= 1;
            return x;
        }
        let x = self.vqb.pop_front().unwrap();
        self.veq();
        self.size -= 1;
        x
    }
    fn vfront(&self) -> i32 {
        if self.size == 0 { return -1; }
        if self.vqf.is_empty() && self.vqb.is_empty() {
            return self.c;
        }
        if self.vqb.is_empty() {
            return self.c;
        }
        *self.vqb.front().unwrap()
    }
    fn vback(&self) -> i32 {
        if self.size == 0 { return -1; }
        if self.vqf.is_empty() && self.vqb.is_empty() {
            return self.c;
        }
        if self.vqf.is_empty() {
            return self.c;
        }
        *self.vqf.back().unwrap()
    }
    fn vsize(&self) -> usize {
        self.vqf.len() + self.vqb.len() + if self.size == 0 { 0 } else { 1 }
    }
    fn size(&self) -> usize {
        self.size
    }
    fn empty(&self) -> u8 {
        if self.size == 0 { 1 } else { 0 }
    }
    fn middle(&self) -> i32 {
        if self.size == 0 { return -1; }
        self.c
    }
    fn debug(&self) {
        println!("\n--- debug ---");
        for e in self.vqf.iter().rev() {
            for _ in 0..self.hqf.len() {
                print!("\t");
            }
            println!("{}", e);
        }
        for e in self.hqf.iter().rev() {
            print!("{}\t", e);
        }
        print!("{}\t", self.c);
        for e in self.hqb.iter() {
            print!("{}\t", e);
        }
        println!("");
        for e in self.vqb.iter() {
            for _ in 0..self.hqf.len() {
                print!("\t");
            }
            println!("{}", e);
        }
        println!("\n--- debug ---");
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut q = Queueueue::new();
    let n = next::<usize>(&mut it);
    for _ in 0..n {
        let cmd = next::<String>(&mut it);
        let cmd = cmd.as_str();
        match cmd {
            "hpush" => q.hpush(next::<i32>(&mut it)),
            "hpop" => writeln!(so, "{}", q.hpop()).unwrap(),
            "hfront" => writeln!(so, "{}", q.hfront()).unwrap(),
            "hback" => writeln!(so, "{}", q.hback()).unwrap(),
            "hsize" => writeln!(so, "{}", q.hsize()).unwrap(),
            "vpush" => q.vpush(next::<i32>(&mut it)),
            "vpop" => writeln!(so, "{}", q.vpop()).unwrap(),
            "vfront" => writeln!(so, "{}", q.vfront()).unwrap(),
            "vback" => writeln!(so, "{}", q.vback()).unwrap(),
            "vsize" => writeln!(so, "{}", q.vsize()).unwrap(),
            "size" => writeln!(so, "{}", q.size()).unwrap(),
            "empty" => writeln!(so, "{}", q.empty()).unwrap(),
            "middle" => writeln!(so, "{}", q.middle()).unwrap(),
            _ => panic!("unknown command"),
        }
        // q.debug();
    }
}