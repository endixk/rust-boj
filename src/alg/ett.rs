fn ett(adj: &Vec<Vec<usize>>, sz: &mut Vec<usize>, map: &mut Vec<usize>, cur: usize, par: usize,
       a: &Vec<i32>, b: &mut Vec<i32>, c: &mut usize) -> usize {
    sz[cur] = 1; map[cur] = *c;
    b[*c] = a[cur]; *c += 1;
    for &nxt in &adj[cur] {
        if nxt == par { continue; }
        sz[cur] += ett(adj, sz, map, nxt, cur, a, b, c);
    }
    sz[cur]
}