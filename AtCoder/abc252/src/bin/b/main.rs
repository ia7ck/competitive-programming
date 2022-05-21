use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u8; n],
        b: [usize; k],
    };

    let max = a.iter().max().copied().unwrap();
    let arg_max: Vec<usize> = (0..n).filter(|&i| a[i] == max).collect();
    for i in arg_max {
        if b.contains(&(i + 1)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
