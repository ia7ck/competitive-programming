use proconio::input;

fn main() {
    input! {
        (x1, y1, z1, x2, y2, z2): (i32, i32, i32, i32, i32, i32),
        (x3, y3, z3, x4, y4, z4): (i32, i32, i32, i32, i32, i32),
    };

    if intersect((x1, x2), (x3, x4)) && intersect((y1, y2), (y3, y4)) && intersect((z1, z2), (z3, z4)) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn intersect((x1, x2): (i32, i32), (x3, x4): (i32, i32)) -> bool {
    x1 < x4 && x3 < x2
}
