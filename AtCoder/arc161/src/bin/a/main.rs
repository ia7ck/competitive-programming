use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    };

    a.sort();
    a.reverse();
    let mut b = vec![0; n];
    for i in 0..(n / 2) {
        b[i * 2 + 1] = a[i];
    }
    for i in (n / 2)..n {
        b[(i - n / 2) * 2] = a[i];
    }

    for i in (1..(n - 1)).step_by(2) {
        if b[i - 1] < b[i] && b[i] > b[i + 1] {
            
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
