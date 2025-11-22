use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
        z: u32,
    };

    if x <= y {
        println!("No");
        return;
    }

    for a in 0..1000 {
        if (x + a) == (y + a) * z {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
