use proconio::input;

fn main() {
    input! {
        a: [u8; 5],
    };

    for i in 0..4 {
        let mut a = a.clone();
        a.swap(i, i + 1);
        if a == vec![1, 2, 3, 4, 5] {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
