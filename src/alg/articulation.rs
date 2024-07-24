fn artp(u: usize, init: bool, adj: &Vec<Vec<usize>>,
        reach: &mut Vec<usize>, it: &mut usize, art: &mut Vec<bool>) -> usize {
    *it += 1;
    reach[u] = *it;
    let mut nc = 0;
    let mut ret = reach[u];
    for &v in &adj[u] {
        if reach[v] != 0 {
            ret = ret.min(reach[v]);
            continue;
        }
        nc += 1;
        let r = artp(v, false, adj, reach, it, art);
        if !init && r >= reach[u] {
            art[u] = true;
        }
        ret = ret.min(r);
    }
    if init && nc > 1 { art[u] = true; }
    ret
}