use proconio::input;

fn score(a: &[u32]) -> u32 {
    assert!(a.len() >= 2);
    let mut a = a.to_vec();
    a.sort();
    a[1..(a.len() - 1)].iter().sum()
}

fn main() {
    input! {
        n: usize,
        x: u32,
        a: [u32; n - 1],
    };

    for last in 0..=100 {
        let mut b = a.clone();
        b.push(last);
        if score(&b) >= x {
            println!("{}", last);
            return;
        }
    }

    println!("-1");
}
