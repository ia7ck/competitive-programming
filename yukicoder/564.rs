use std::io::Read;

fn read<T: std::str::FromStr>() -> T {
    let token: String = std::io::stdin()
        .bytes()
        .map(|c| c.ok().unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

mod binary_search {
    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::ops::Range;
    pub trait BinarySearch<T> {
        fn lower_bound(&self, x: &T) -> usize;
        fn upper_bound(&self, x: &T) -> usize;
        fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>);
    }

    // min index self[i] >= x
    // that is, any j (j < i) holds self[j] < x
    impl<T: Ord> BinarySearch<T> for [T] {
        fn lower_bound(&self, x: &T) -> usize {
            match self[0].cmp(x) {
                Greater | Equal => {
                    return 0;
                }
                _ => (),
            }
            let mut lf = 0;
            let mut rg = self.len();
            // self[lf] < x
            while rg - lf > 1 {
                let md = (rg + lf) / 2;
                match self[md].cmp(x) {
                    Less => {
                        lf = md;
                    }
                    Greater | Equal => {
                        rg = md;
                    }
                }
            }
            rg
        }

        // min index self[i] > x
        // that is, any j (j < i) holds self[j] <= x
        fn upper_bound(&self, x: &T) -> usize {
            match self[0].cmp(x) {
                Greater => {
                    return 0;
                }
                _ => (),
            }
            let mut lf = 0;
            let mut rg = self.len();
            // self[lf] <= x
            while rg - lf > 1 {
                let md = (rg + lf) / 2;
                match self[md].cmp(x) {
                    Less | Equal => {
                        lf = md;
                    }
                    Greater => {
                        rg = md;
                    }
                }
            }
            rg
        }

        fn split_by(&self, x: &T) -> (Range<usize>, Range<usize>, Range<usize>) {
            let i = self.lower_bound(x);
            let j = self.upper_bound(x);
            (0..i, i..j, j..self.len())
        }
    }
}

use binary_search::BinarySearch;

fn main() {
    let h: i32 = read();
    let n: usize = read();
    let mut a = vec![0; n - 1];
    for i in 0..n - 1 {
        a[i] = read();
    }
    a.sort();
    let (_, _, r) = a.split_by(&h);
    let o = r.len() + 1;
    println!(
        "{}",
        if o % 10 == 1 {
            format!("{}st", o)
        } else if o % 10 == 2 {
            format!("{}nd", o)
        } else if o % 10 == 3 {
            format!("{}rd", o)
        } else {
            format!("{}th", o)
        }
    );
}
