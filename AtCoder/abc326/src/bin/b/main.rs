use proconio::input;

fn main() {
    input! {
        n: u16,
    };

    for k in n..1000 {
        let a = k % 10;
        let b = k / 10 % 10;
        let c = k / 100 % 10;
        if b * c == a {
            println!("{}", k);
            return;
        }
    }

    unreachable!();
}
