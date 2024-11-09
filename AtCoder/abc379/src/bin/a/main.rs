use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    };

    let mut ans1 = n.clone();
    let mut ans2 = n.clone();
    ans1.rotate_left(1);
    ans2.rotate_right(1);
    let ans1 = ans1.iter().collect::<String>();
    let ans2 = ans2.iter().collect::<String>();
    
    println!("{} {}", ans1, ans2);
}
