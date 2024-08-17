use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
        c: u8,
    };

    let ng = if b < c {
        b < a && a < c
    } else {
        b < a || a < c
    };

    if ng {
        println!("No");
    } else {
        println!("Yes");
    }
}
