#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Query {
    s: usize,
    e: usize,
    k: usize,
    i: usize,
}
impl Query {
    fn new(s: usize, e: usize, n: usize, i: usize) -> Self {
        Self { s, e, k: (n as f64).sqrt() as usize, i }
    }
}
impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.s / self.k).cmp(&(other.s / other.k))
            .then(self.e.cmp(&other.e))
    }
}
impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const MAX: usize = 12345;
fn add(cnt: &mut Vec<usize>, v: &mut usize, x: usize) {
    unimplemented!();
}
fn del(cnt: &mut Vec<usize>, v: &mut usize, x: usize) {
    unimplemented!();
}
fn mo(ans: &mut Vec<usize>, arr: &[usize], qry: &[Query]) {
    let mut cnt = vec![0; MAX+1];
    let mut v = 0;
    for i in qry[0].s..=qry[0].e {
        add(&mut cnt, &mut v, arr[i]);
    }
    ans[qry[0].i] = v;

    let (mut s, mut e) = (qry[0].s, qry[0].e);
    for &q in qry.iter().skip(1) {
        while q.s < s { add(&mut cnt, &mut v, arr[s - 1]); s -= 1; }
        while e < q.e { add(&mut cnt, &mut v, arr[e + 1]); e += 1; }
        while s < q.s { del(&mut cnt, &mut v, arr[s]); s += 1; }
        while q.e < e { del(&mut cnt, &mut v, arr[e]); e -= 1; }
        ans[q.i] = v;
    }
}
