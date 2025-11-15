use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
    };

    x.sort_unstable();

    let pos = x.iter().position(|&c| c != '0').unwrap();
    let head = x[pos];
    x.remove(pos);
    x.insert(0, head);

    let ans: String = x.iter().collect();
    println!("{}", ans);
}
