extern crate proconio;
use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    println!(
        "{}",
        if 400 <= x && x < 600 {
            8
        } else if x < 800 {
            7
        } else if x < 1000 {
            6
        } else if x < 1200 {
            5
        } else if x < 1400 {
            4
        } else if x < 1600 {
            3
        } else if x < 1800 {
            2
        } else if x < 2000 {
            1
        } else {
            unreachable!();
        }
    );
}
