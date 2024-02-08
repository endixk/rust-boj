// BOJ 6590 [Addition Chains]
pub fn main() { read();
    let v: Vec<Vec<u8>> = vec![
        vec![1],
        vec![1,2],
        vec![1,2,3],
        vec![1,2,4],
        vec![1,2,3,5],
        vec![1,2,3,6],
        vec![1,2,3,4,7],
        vec![1,2,4,8],
        vec![1,2,3,6,9],
        vec![1,2,3,5,10],
        vec![1,2,3,4,7,11],
        vec![1,2,3,6,12],
        vec![1,2,3,5,8,13],
        vec![1,2,3,4,7,14],
        vec![1,2,3,5,10,15],
        vec![1,2,4,8,16],
        vec![1,2,4,8,9,17],
        vec![1,2,3,6,9,18],
        vec![1,2,3,4,8,11,19],
        vec![1,2,3,5,10,20],
        vec![1,2,3,4,7,14,21],
        vec![1,2,3,4,7,11,22],
        vec![1,2,3,5,10,13,23],
        vec![1,2,3,6,12,24],
        vec![1,2,3,5,10,15,25],
        vec![1,2,3,5,8,13,26],
        vec![1,2,3,6,9,18,27],
        vec![1,2,3,4,7,14,28],
        vec![1,2,3,4,7,11,18,29],
        vec![1,2,3,5,10,15,30],
        vec![1,2,3,4,7,14,17,31],
        vec![1,2,4,8,16,32],
        vec![1,2,4,8,16,17,33],
        vec![1,2,4,8,9,17,34],
        vec![1,2,3,4,7,14,21,35],
        vec![1,2,3,6,9,18,36],
        vec![1,2,3,5,8,16,21,37],
        vec![1,2,3,4,8,11,19,38],
        vec![1,2,3,5,8,13,26,39],
        vec![1,2,3,5,10,20,40],
        vec![1,2,3,5,10,20,21,41],
        vec![1,2,3,4,7,14,21,42],
        vec![1,2,3,5,10,20,23,43],
        vec![1,2,3,4,7,11,22,44],
        vec![1,2,3,5,10,15,30,45],
        vec![1,2,3,5,10,13,23,46],
        vec![1,2,3,4,7,10,20,27,47],
        vec![1,2,3,6,12,24,48],
        vec![1,2,3,6,12,24,25,49],
        vec![1,2,3,5,10,15,25,50],
        vec![1,2,3,6,12,24,27,51],
        vec![1,2,3,5,8,13,26,52],
        vec![1,2,3,5,6,12,24,29,53],
        vec![1,2,3,6,9,18,27,54],
        vec![1,2,3,4,7,11,22,33,55],
        vec![1,2,3,4,7,14,28,56],
        vec![1,2,3,4,7,14,28,29,57],
        vec![1,2,3,4,7,11,18,29,58],
        vec![1,2,3,4,7,14,28,31,59],
        vec![1,2,3,5,10,15,30,60],
        vec![1,2,3,5,7,14,28,33,61],
        vec![1,2,3,4,7,14,17,31,62],
        vec![1,2,3,4,7,14,21,42,63],
        vec![1,2,4,8,16,32,64],
        vec![1,2,4,8,16,32,33,65],
        vec![1,2,4,8,16,17,33,66],
        vec![1,2,3,4,8,16,32,35,67],
        vec![1,2,4,8,9,17,34,68],
        vec![1,2,3,5,8,16,32,37,69],
        vec![1,2,3,4,7,14,21,35,70],
        vec![1,2,3,4,7,8,16,32,39,71],
        vec![1,2,3,6,9,18,36,72],
        vec![1,2,3,6,9,18,36,37,73],
        vec![1,2,3,5,8,16,21,37,74],
        vec![1,2,3,5,10,15,25,50,75],
        vec![1,2,3,4,8,11,19,38,76],
        vec![1,2,4,5,9,18,36,41,77],
        vec![1,2,3,5,8,13,26,39,78],
        vec![1,2,3,4,7,9,18,36,43,79],
        vec![1,2,3,5,10,20,40,80],
        vec![1,2,3,5,10,20,40,41,81],
        vec![1,2,3,5,10,20,21,41,82],
        vec![1,2,3,5,10,20,40,43,83],
        vec![1,2,3,4,7,14,21,42,84],
        vec![1,2,3,5,10,20,40,45,85],
        vec![1,2,3,5,10,20,23,43,86],
        vec![1,2,3,4,7,10,20,40,47,87],
        vec![1,2,3,4,7,11,22,44,88],
        vec![1,2,3,4,7,11,22,44,45,89],
        vec![1,2,3,5,10,15,30,45,90],
        vec![1,2,3,4,7,11,22,44,47,91],
        vec![1,2,3,5,10,13,23,46,92],
        vec![1,2,3,4,7,14,17,31,62,93],
        vec![1,2,3,4,7,10,20,27,47,94],
        vec![1,2,3,4,7,11,22,44,51,95],
        vec![1,2,3,6,12,24,48,96],
        vec![1,2,3,6,12,24,48,49,97],
        vec![1,2,3,6,12,24,25,49,98],
        vec![1,2,3,6,12,24,48,51,99],
        vec![1,2,3,5,10,15,25,50,100]
    ];

    loop {
        let n = next::<usize>();
        if n == 0 { break; }
        println!("{}", v[n-1].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}