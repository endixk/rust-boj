use std::io::{self, Read};
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

use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cmp::Reverse;
const INF: usize = 0x3f3f3f3f;
fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dist[src] = 0;
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d { continue; }
        for &(v, w) in &adj[u] {
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                pq.push((Reverse(dist[v]), v));
            }
        }
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut arr = [99; 8];
    for i in 0..n {
        arr[i] = next::<u8>(&mut it);
    }

    let m = next::<usize>(&mut it);
    let swap = (0..m).map(|_| {
        let i = next::<usize>(&mut it);
        let j = next::<usize>(&mut it);
        let c = next::<usize>(&mut it);
        (i-1, j-1, c)
    }).collect::<Vec<_>>();

    let mut map = HashMap::new();
    map.insert(arr.clone(), 0usize);
    let mut q = VecDeque::new();
    q.push_back(arr);
    let mut adj = vec![];
    let mut id = 1;
    while let Some(arr) = q.pop_front() {
        let mut v = vec![];
        for &(i, j, c) in &swap {
            let mut nxt = arr.clone();
            nxt.swap(i, j);
            if let Some(&k) = map.get(&nxt) {
                v.push((k, c));
            } else {
                map.insert(nxt.clone(), id);
                q.push_back(nxt);
                v.push((id, c));
                id += 1;
            }
        }
        adj.push(v);
    }

    let mut dist = vec![INF; id];
    dijkstra(&adj, &mut dist, 0);
    arr.sort_unstable();
    if let Some(&ans) = map.get(&arr) {
        println!("{}", dist[ans]);
    } else {
        println!("-1");
    }
}