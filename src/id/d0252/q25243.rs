// BOJ 25243 [Jungbunaeryuk Line]
#[derive(Clone, Copy)] struct Train {
    idx: usize, // train index
    dept: i32, // departure time
    id: i32, // train id
    south: bool, // train is southbound
    track: usize, // track number
    rem: i32, // remaining time
    arrv: i32, // arrival time
}
#[derive(Clone)] struct Station {
    arrv: Vec<usize>, // arrived train ids
    pend: Vec<usize>, // pending train ids
}
fn ttoi(t: &String) -> i32 {
    let h = t[0..2].parse::<i32>().unwrap();
    let m = t[3..5].parse::<i32>().unwrap();
    h * 60 + m
}
fn itot(i: i32) -> String {
    let i = (i + 1440) % 1440;
    format!("{:02}:{:02}", i / 60, i % 60)
}
pub fn main() { read();
    let mut line = vec![Station { arrv: vec![], pend: vec![] }; 5];
    let treq = vec![7, 7, 8, 10]; // time required for each track
    let mut tvac = vec![true; 4]; // track availability
    let mut trains = vec![];

    let (c, h) = (next::<usize>(), next::<usize>());
    for idx in 0..c {
        let (id, dept) = (next::<i32>(), ttoi(&next()));
        trains.push(Train { idx, dept, id, south: true, track: 0, rem: 0, arrv: -1 });
    }
    for idx in c..c+h {
        let (id, dept) = (next::<i32>(), ttoi(&next()));
        trains.push(Train { idx, dept, id, south: false, track: 0, rem: 0, arrv: -1 });
    }

    let (mut proc, mut time) = (0, 0);
    while proc < c + h {
        // step 1. put departing trains to pending list
        for train in trains.iter_mut() {
            if train.dept == time {
                line[if train.south { 0 } else { 4 }].pend.push(train.idx);
            }
        }
        // step 2. process pending trains
        for tid in 0..4 {
            if !tvac[tid] { continue; }
            let (mut idx, mut mdept, mut mid) = (99, 1440, 1000);
            for &id in &line[tid].pend { // check southbound trains
                let train = &trains[id];
                if !train.south { continue; }
                if train.dept < mdept || (train.dept == mdept && train.id < mid) {
                    (idx, mdept, mid) = (id, train.dept, train.id);
                }
            }
            for &id in &line[tid + 1].pend { // check northbound trains
                let train = &trains[id];
                if train.south { continue; }
                if train.dept < mdept || (train.dept == mdept && train.id < mid) {
                    (idx, mdept, mid) = (id, train.dept, train.id);
                }
            }
            if idx == 99 { continue; }

            // load train to track
            let train = &mut trains[idx];
            if train.south { line[tid].pend.retain(|&x| x != idx); }
            else { line[tid + 1].pend.retain(|&x| x != idx); }
            train.rem = treq[tid];
            train.track = tid;
            tvac[tid] = false;
        }
        // step 3. process arrived trains
        for station in line.iter_mut() {
            station.pend.extend(station.arrv.drain(..));
        }
        // step 4. move trains
        for train in trains.iter_mut() {
            if train.rem > 0 {
                train.rem -= 1;
                if train.rem == 0 {
                    tvac[train.track] = true;
                    if train.south {
                        if train.track == 3 {
                            train.arrv = time + 1;
                            proc += 1;
                        } else {
                            line[train.track + 1].arrv.push(train.idx);
                        }
                    } else {
                        if train.track == 0 {
                            train.arrv = time + 1;
                            proc += 1;
                        } else {
                            line[train.track].arrv.push(train.idx);
                        }
                    }
                }
            }
        }
        // step 5. update time
        time += 1;
    }

    trains.sort_by_key(|x| x.id);
    for train in trains.iter() {
        println!("{}", itot(train.arrv));
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