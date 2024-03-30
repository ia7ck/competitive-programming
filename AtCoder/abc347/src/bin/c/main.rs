use proconio::input;

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        d: [u64; n],
    };

    let mut d = d.into_iter().map(|d| d % (a + b)).collect::<Vec<_>>();
    d.sort();
    d.dedup();
    if d.len() == 1 {
        println!("Yes");
        return;
    }
    for i in 0..d.len() {
        // d[i] .. d[i+1] : weekday
        assert!((a + b) + d[(i + 1) % d.len()] >= d[i]);
        if ((a + b) + d[(i + 1) % d.len()] - d[i]) % (a + b) > b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
