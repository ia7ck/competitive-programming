use proconio::input;

fn main() {
    input! {
        x: u64,
    };

    for n in 2.. {
        let f = (1..=n).fold(1, |acc, i| acc * i);
        if f == x {
            println!("{}", n);
            break;
        }
    }
}
