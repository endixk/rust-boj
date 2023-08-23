#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}
static mut ORI: Point = Point { x: 0, y: 0 };
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            ccw(&ORI, other, self).cmp(&0)
                .then(dist(&ORI, self).cmp(&dist(&ORI, other)))
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) as i64 * (c.y - a.y) as i64 - (b.y - a.y) as i64 * (c.x - a.x) as i64;
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}
fn dist(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x) as i64).pow(2) + ((a.y - b.y) as i64).pow(2)
}

fn graham(mut points: Vec<Point>) -> Vec<Point> {
    // find the lowest point
    let mut loc = 0;
    for i in 1..points.len() {
        if points[i].y < points[loc].y || (points[i].y == points[loc].y && points[i].x < points[loc].x) {
            loc = i;
        }
    }
    points.swap(0, loc);
    unsafe { ORI = points[0].clone(); }

    // sort the points by polar angle
    points.sort_unstable();

    // find the convex hull
    let mut hull = Vec::new();
    hull.push(points[0]);
    hull.push(points[1]);
    for i in 2..points.len() {
        while hull.len() >= 2 {
            let a = hull.pop().unwrap();
            let b = hull.last().unwrap();
            if ccw(b, &a, &points[i]) > 0 {
                hull.push(a);
                break;
            }
        }
        hull.push(points[i]);
    }

    hull
}

fn cross(x1: &Point, x2: &Point, y1: &Point, y2: &Point) -> i64 {
    (x2.x - x1.x) as i64 * (y2.y - y1.y) as i64 - (x2.y - x1.y) as i64 * (y2.x - y1.x) as i64
}
fn calipers(hull: Vec<Point>) -> (Point, Point) {
    // find the leftmost and rightmost points
    let (mut l, mut r) = (0, 0);
    hull.iter().enumerate().skip(1).for_each(|(i, p)| {
        if p.x < hull[l].x { l = i; }
        if p.x > hull[r].x { r = i; }
    });

    // find the farthest distance
    let mut d = dist(&hull[l], &hull[r]);
    let mut ret = (hull[l].clone(), hull[r].clone());
    for _ in 1..hull.len() {
        // rotate
        if cross(&hull[l], &hull[(l + 1) % hull.len()], &hull[r], &hull[(r + 1) % hull.len()]) < 0 {
            l = (l + 1) % hull.len();
        } else {
            r = (r + 1) % hull.len();
        }
        // update
        let t = dist(&hull[l], &hull[r]);
        if t > d {
            d = t;
            ret = (hull[l].clone(), hull[r].clone());
        }
    }

    ret
}