#[derive(Debug, Clone)] struct Ch {
    name: String,
    group: String,
    team: String,
    p: usize,
    r: usize,
}
#[inline] fn tier(k: usize) -> usize {
    match k {
        k if k < 17 => 1,
        k if k < 33 => 2,
        k if k < 49 => 3,
        k if k < 65 => 4,
        k if k < 81 => 5,
        _ => 6,
    }
}
pub fn main() { read();
    let info = unsafe { IT.as_mut().unwrap().map(|s| s.to_string()).collect::<Vec<_>>() };
    let mut chs = vec![];
    let mut map = std::collections::HashMap::new();

    let mut i = 0;
    for k in 1..=80 {
        let mut name = info[i].to_string();
        i += 1;
        while info[i] != "Group" {
            name += " ";
            name += &info[i];
            i += 1;
        }
        let group = info[i+1].to_string();
        let team = info[i+3].to_string();
        chs.push(Ch { name: name.clone(), group: group.clone(), team: team.clone(), p: k, r: 81 });
        map.insert(name + &group + &team, k);
        i += 4;
    }

    for k in 1..=80 {
        let mut name = info[i].to_string();
        i += 1;
        while info[i] != "Group" {
            name += " ";
            name += &info[i];
            i += 1;
        }
        let group = info[i+1].to_string();
        let team = info[i+3].to_string();
        if map.contains_key(&(name.clone() + &group + &team)) {
            chs[map[&(name.clone() + &group + &team)]-1].r = k;
        } else {
            chs.push(Ch { name: name.clone(), group: group.clone(), team: team.clone(), p: 81, r: k });
        }
        i += 4;
    }

    let mut cin = vec![];
    for ch in &chs {
        if tier(ch.r) == 6 {
            cin.push((0, ch.clone()));
            continue;
        }

        let mut c = 0;
        if tier(ch.p) > tier(ch.r) {
            c += 10000 * (tier(ch.p) - tier(ch.r));
            if ch.r == 1 || ch.r == 17 || ch.r == 33 || ch.r == 49 || ch.r == 65 { c += 10000; }
        }
        if ch.r == 1 || ch.r == 17 || ch.r == 33 || ch.r == 49 || ch.r == 65 { c += 10000; }
        if tier(ch.p) != 1 && tier(ch.r) == 1 {
            let mut cnt = 0;
            for ch2 in &chs {
                if (tier(ch2.p) == 1 && tier(ch2.r) != 1) && (ch2.p < ch.p && ch2.r > ch.r) { cnt += 1; }
            }
            c += 30000 * cnt;
        }
        cin.push((c, ch.clone()));
    }

    cin.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.r.cmp(&b.1.r)));
    println!("{}", cin[0].1.group);
    println!("{}", cin[0].1.team);
    println!("{}", cin[0].1.name);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}