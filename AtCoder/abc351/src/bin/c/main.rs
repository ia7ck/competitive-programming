use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };

    let mut b = Vec::new();
    for x in a {
        b.push(x);
        while b.len() >= 2 {
            let k = b.len();
            let y = b[k - 1];
            let z = b[k - 2];
            if y != z {
                break;
            }
            b.pop(); // y
            b.pop(); // z
            b.push(y + 1); // z + 1
        }
    }

    let ans = b.len();
    println!("{}", ans);
}
