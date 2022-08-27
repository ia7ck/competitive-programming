use proconio::input;

fn cross(x: i64, y: i64, xx: i64, yy: i64) -> i64 {
    x * yy - y * xx
}

fn ok(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> bool {
    cross(x3 - x2, y3 - y2, x1 - x2, y1 - y2) > 0
}

fn main() {
    input! {
        ax: i64,
        ay: i64,
        bx: i64,
        by: i64,
        cx: i64,
        cy: i64,
        dx: i64,
        dy: i64,
    };

    if ok(ax, ay, bx, by, cx, cy)
        && ok(bx, by, cx, cy, dx, dy)
        && ok(cx, cy, dx, dy, ax, ay)
        && ok(dx, dy, ax, ay, bx, by)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
