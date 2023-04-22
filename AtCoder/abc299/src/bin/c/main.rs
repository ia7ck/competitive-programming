use proconio::{input, marker::Chars};
use run_length::RunLength;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };

    let rle = RunLength::new(s.iter()).collect::<Vec<_>>();
    let mut x = 0;
    for w in rle.windows(2) {
        let (c1, l1) = w[0];
        let (c2, l2) = w[1];
        if c1 == &'o' {
            x = x.max(l1);
        } else {
            assert_eq!(c2, &'o');
            x = x.max(l2);
        }
    }

    if x == 0 {
        println!("-1");
    } else {
        println!("{}", x);
    }
}
