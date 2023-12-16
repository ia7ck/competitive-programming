use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut repunit = vec![1_u64];
    for i in 0..14 {
        repunit.push(repunit[i] * 10 + 1);
    }
    let mut values = Vec::new();
    for &x in &repunit {
        for &y in &repunit {
            for &z in &repunit {
                values.push(x + y + z);
            }
        }
    }
    values.sort();
    values.dedup();
    assert!(values.len() >= n);
    println!("{}", values[n - 1]);
}
