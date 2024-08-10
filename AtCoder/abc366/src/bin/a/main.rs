use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    };

    let rest = n - (t + a);
    if t + rest > a && t < a + rest {
        println!("No");
    } else {
        println!("Yes");
    }
}
