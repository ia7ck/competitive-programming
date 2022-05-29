use proconio::input;

fn main() {
    input! {
        mut x: [u16; 3],
    };

    let b = x[1];
    x.sort();
    if b == x[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
