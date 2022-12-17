use proconio::input;

fn main() {
    input! {
        k: usize,
    };

    let mut ans = String::new();
    for i in 0..k {
        let ch = 'A' as u8 + i as u8;
        ans.push(ch as char);
    }

    println!("{}", ans);
}
