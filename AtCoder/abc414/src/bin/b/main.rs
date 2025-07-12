use proconio::input;

fn main() {
    input! {
        n: usize,
        cl: [(char, usize); n],
    };

    let mut s = String::new();
    for (c, l) in cl {
        if s.len() + l > 100 {
            println!("Too Long");
            return;
        }
        for _ in 0..l {
            s.push(c);
        }
    }

    println!("{}", s);
}