use proconio::input;

fn main() {
    input! {
        (xa, ya): (i32, i32),
        (xb, yb): (i32, i32),
        (xc, yc): (i32, i32),
    };

    if check((xa, ya), (xb, yb), (xc, yc))
        || check((xb, yb), (xc, yc), (xa, ya))
        || check((xc, yc), (xa, ya), (xb, yb))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn check((xa, ya): (i32, i32), (xb, yb): (i32, i32), (xc, yc): (i32, i32)) -> bool {
    let d_ab = (xb - xa).pow(2) + (yb - ya).pow(2);
    let d_ac = (xc - xa).pow(2) + (yc - ya).pow(2);
    let d_bc = (xc - xb).pow(2) + (yc - yb).pow(2);
    d_ab + d_ac == d_bc
}
