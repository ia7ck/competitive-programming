use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
        a: [u32; n],
    };

    for i in 0..n {
        let mut sum = 0;
        for j in 0..n {
            if i != j {
                sum += a[j];
            }
        }
        if sum == m {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
