use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, dist: &mut Vec<usize>, src: usize) {
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), src));
    dist[src] = 0;
    while let Some((Reverse(d), u)) = pq.pop() {
        if dist[u] < d { continue; }
        for (v, w) in &adj[u] {
            if dist[*v] > dist[u] + *w {
                dist[*v] = dist[u] + *w;
                pq.push((Reverse(dist[*v]), *v));
            }
        }
    }
}