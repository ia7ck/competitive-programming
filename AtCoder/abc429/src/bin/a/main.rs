use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    for i in 0..n {
        if i < m {
            println!("OK");
        } else {
            println!("Too Many Requests");
        }
    }
}
