use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64,
    };

    let mut a = vec![0, x, y];
    for i in 3..=10 {
        a.push(f(a[i - 1] + a[i - 2]));
    }
    println!("{}", a[10]);
}

fn f(x: u64) -> u64 {
    let mut rev_x = x.to_string().bytes().collect::<Vec<_>>();
    rev_x.reverse();
    let mut y = 0;
    for d in rev_x {
        y = y * 10 + u64::from(d - b'0');
    }
    y
}
