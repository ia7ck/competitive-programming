use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32; n],
    };

    if x.iter().all(|&x| x < 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
