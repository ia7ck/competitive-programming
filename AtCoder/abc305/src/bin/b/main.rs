use proconio::input;

fn main() {
    input! {
        p: char,
        q: char,
    };

    let (p, q) = if p < q {
        (p, q)
    } else {
        (q, p)
    };
    let index = |ch: char| {
        ch as usize - 'A' as usize
    };
    let mut x = [0, 3, 1, 4, 1, 5, 9];
    for i in 1..x.len() {
        x[i] += x[i - 1];
    }
    let ans = x[index(q)] - x[index(p)];
    println!("{}", ans);
}
