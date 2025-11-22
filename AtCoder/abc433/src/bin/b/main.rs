use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    for i in 0..n {
        if let Some(j) = a[..i].iter().rposition(|&x| x > a[i]) {
            println!("{}", j + 1);
        } else {
            println!("-1");
        }
    }
}
