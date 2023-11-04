use proconio::input;

fn pow(x: u128, y: u128) -> u128 {
    if y == 0 {
        1
    } else if y % 2 == 0 {
        pow(x * x, y / 2)
    } else {
        x * pow(x, y - 1)
    }
}

fn main() {
    input! {
        b: u128,
    };

    for a in 1.. {
        let p = pow(a, a);
        if p > b {
            break;
        }
        if p == b {
            println!("{}", a);
            return;
        }
    }
    println!("-1");
}
