use proconio::{input, marker::Bytes};
use rolling_hash::RollingHash;

fn main() {
    input! {
        n: usize,
        t: Bytes,
    };

    let mut rt = t.clone();
    rt.reverse();
    let t: Vec<u64> = t.into_iter().map(|b| u64::from(b)).collect();
    let rt: Vec<u64> = rt.into_iter().map(|b| u64::from(b)).collect();
    let rh = RollingHash::new(&t);
    let rrh = RollingHash::new(&rt);

    for i in 0..=n {
        // eprintln!("i = {}", i);
        // eprintln!("{:?}, {:?}", &t[0..i], &t[(n * 2 - (n - i))..(n * 2)]);
        let u = rh.connect(rh.get(0..i), rh.get((n * 2 - (n - i))..(n * 2)), n - i);
        let v = rrh.get((n - i)..(n - i + n));
        if u == v {
            for &b in &rt[(n - i)..(n - i + n)] {
                let ch = b as u8 as char;
                print!("{}", ch);
            }
            println!();
            println!("{}", i);
            return;
        }
    }

    println!("-1");
}
