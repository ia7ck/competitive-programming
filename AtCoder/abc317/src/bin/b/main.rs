use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let min = a.iter().min().copied().unwrap();
    let max = a.iter().max().copied().unwrap();
    assert_eq!(max - min, n as u32);
    for x in min..=max {
        if a.contains(&x) == false {
            println!("{}", x);
            return;
        }
    }

    unreachable!();
}
