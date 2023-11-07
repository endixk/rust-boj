#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[inline] fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}

fn inside_convex(poly: &Vec<Point>, p: &Point, n: usize) -> bool {
    if ccw(&poly[0], &poly[1], p) < 0 || ccw(&poly[0], &poly[n-1], p) > 0 {
        return false;
    }
    let (mut l, mut r) = (1, n-1);
    while l+1 < r {
        let m = (l+r) / 2;
        if ccw(&poly[0], &poly[m], p) > 0 { l = m; }
        else { r = m; }
    }
    ccw(&poly[l], &poly[r], p) > 0
}

#[inline] fn within(a: i32, b: i32, x: i32) -> bool {
    return if a < b { a <= x && x <= b } else { b <= x && x <= a }
}
fn inside_non_convex(poly: &Vec<Point>, p: &Point, n: usize) -> bool {
    let mut cnt = 0;
    for i in 0..poly.len() {
        let j = (i+1) % poly.len();
        let (x, y) = if poly[i].y < poly[j].y { (&poly[i], &poly[j]) } else { (&poly[j], &poly[i]) };

        if ccw(x, y, p) == 0 && within(x.x, y.x, p.x) && within(x.y, y.y, p.y) {
            return true; // p is on the line
        }
        if x.y <= p.y && p.y < y.y && ccw(x, y, p) > 0 {
            cnt += 1;
        }
    }
    cnt & 1 == 1
}