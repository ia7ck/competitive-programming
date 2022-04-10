use join::Join;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut s = vec![];
    for i in 1..=n {
        let mut t = Vec::new();
        t.extend(s.clone());
        t.push(i);
        t.extend(s);
        s = t;
    }
    println!("{}", s.iter().join(" "));
}
