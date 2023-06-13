// BOJ 2733 [Brainf*ck]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

struct BF {
    cmd: Vec<char>,
    mem: [u8; 32768],
    ptr: usize,
    loop_goto: HashMap<usize, usize>,
    loop_back: HashMap<usize, usize>,
}
impl BF {
    fn new(cmd: String) -> Option<Self> {
        let mut loop_goto = HashMap::new();
        let mut loop_back = HashMap::new();

        let mut st = Vec::new();
        let mut comment = false;
        for (i, c) in cmd.chars().enumerate() {
            if !comment && c == '%' { comment = true; continue; }
            if comment && c == '\n' { comment = false; continue; }
            if comment { continue; }
            match c {
                '[' => st.push(i),
                ']' => {
                    let j = st.pop();
                    if j.is_none() { return None; }
                    let j = j.unwrap();
                    loop_goto.insert(j, i);
                    loop_back.insert(i, j);
                },
                _ => (),
            }
        }

        if !st.is_empty() { None }
        else {
            Some(Self {
                cmd: cmd.chars().collect(),
                mem: [0; 32768],
                ptr: 0,
                loop_goto,
                loop_back,
            })
        }
    }

    // >
    fn ptr_incr(&mut self) {
        self.ptr += 1;
        if self.ptr == 32768 { self.ptr = 0; }
    }
    // <
    fn ptr_decr(&mut self) {
        if self.ptr == 0 { self.ptr = 32768; }
        self.ptr -= 1;
    }
    // +
    fn mem_incr(&mut self) {
        if self.mem[self.ptr] == 255 { self.mem[self.ptr] = 0; }
        else { self.mem[self.ptr] += 1; }
    }
    // -
    fn mem_decr(&mut self) {
        if self.mem[self.ptr] == 0 { self.mem[self.ptr] = 255; }
        else { self.mem[self.ptr] -= 1; }
    }
    // .
    fn print(&mut self) -> char {
        self.mem[self.ptr] as char
    }
    // [
    fn loop_start(&mut self, i: usize) -> usize {
        if self.mem[self.ptr] == 0 {
            self.loop_goto[&i]
        } else {
            i
        }
    }
    // ]
    fn loop_end(&mut self, i: usize) -> usize {
        if self.mem[self.ptr] != 0 {
            self.loop_back[&i]
        } else {
            i
        }
    }

    fn go(&mut self) -> String {
        let mut ret = String::new();
        let mut i = 0;
        let mut comment = false;
        while i < self.cmd.len() {
            let c = self.cmd[i];
            if !comment && c == '%' { comment = true; i += 1; continue; }
            if comment && c == '\n' { comment = false; i += 1; continue; }
            if comment { i += 1; continue; }
            match c {
                '>' => self.ptr_incr(),
                '<' => self.ptr_decr(),
                '+' => self.mem_incr(),
                '-' => self.mem_decr(),
                '.' => ret.push(self.print()),
                '[' => i = self.loop_start(i),
                ']' => i = self.loop_end(i),
                _ => (),
            }
            i += 1;
        }

        ret
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let n = s.parse::<usize>().unwrap();

    for i in 1..=n {
        writeln!(so, "PROGRAM #{}:", i).ok();
        let mut cmd = String::new();
        loop {
            let s = read(&mut si);
            if s == "end" { break; }
            cmd.push_str(&s);
            cmd.push('\n');
        }
        let bf = BF::new(cmd);
        if bf.is_none() {
            writeln!(so, "COMPILE ERROR").ok();
        } else {
            let mut bf = bf.unwrap();
            let ret = bf.go();
            writeln!(so, "{}", ret).ok();
        }
    }
}
