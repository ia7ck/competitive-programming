use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    };

    let x = (1..=3).filter(|&x| x != a && x != b).collect::<Vec<u8>>();
    if x.len() == 1 {
        println!("{}", x[0]);
    } else {
        println!("-1");
    }
}
