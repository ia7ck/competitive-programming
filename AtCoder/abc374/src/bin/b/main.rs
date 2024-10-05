use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    if s == t {
        println!("0");
        return;
    }

    for i in 0..s.len().max(t.len()) {
        if s.get(i) != t.get(i) {
            println!("{}", i + 1);
            return;
        }
    }

    unreachable!()
}
