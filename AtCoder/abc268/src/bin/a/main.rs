use proconio::input;

fn main() {
    input! {
        mut a: [u32; 5],
    };

    a.sort();
    a.dedup();
    println!("{}", a.len());
}
