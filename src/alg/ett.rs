fn ett(adj: &Vec<Vec<usize>>, cur: usize, x: &mut usize,
       id: &mut [usize], sz: &mut [usize]) -> usize {
    id[cur] = *x; *x += 1;
    sz[cur] = 1;
    for &nxt in adj[cur].iter() {
        sz[cur] += ett(adj, nxt, x, id, sz);
    }
    sz[cur]
}