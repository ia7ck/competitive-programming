use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    };

    let mut c = Vec::new();
    for i in 0..n {
        c.push((a[i], 'a'));
    }
    for i in 0..m {
        c.push((b[i], 'b'));
    }
    c.sort();

    for w in c.windows(2) {
        let (_, c1) = w[0];
        let (_, c2) = w[1];
        if c1 == 'a' && c2 == 'a' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
