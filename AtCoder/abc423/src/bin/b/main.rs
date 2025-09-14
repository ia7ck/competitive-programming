use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [u8; n],
    };

    if l.iter().all(|&l| l == 0) {
        println!("0");
        return;
    }

    let head = l.iter().take_while(|&&l| l == 0).count();
    let tail = l.iter().rev().take_while(|&&l| l == 0).count();
    let ans = (n + 1) - (head + 1) - (tail + 1);
    println!("{}", ans);
}
