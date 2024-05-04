use proconio::input;

fn main() {
    input! {
        _n: usize,
        x: usize,
        y: usize,
        z: usize,
    };

    if (x <= z && z <= y) || (y <= z && z <= x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
