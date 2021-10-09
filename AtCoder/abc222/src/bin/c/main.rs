use input_i_scanner::{scan_with, InputIScanner};
use std::cmp::Reverse;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (n, m) = scan_with!(_i_i, (usize, usize));
    let mut a = vec![vec![]; n * 2];
    for i in 0..(n * 2) {
        let s = scan_with!(_i_i, String);
        a[i] = s.chars().collect();
    }

    let mut ord: Vec<usize> = (0..(n * 2)).collect();
    let mut win = vec![0; n * 2];
    for i in 0..m {
        for k in 0..n {
            // k*2, k*2+1
            let l = ord[k * 2];
            let r = ord[k * 2 + 1];
            match (a[l][i], a[r][i]) {
                ('G', 'C') | ('C', 'P') | ('P', 'G') => {
                    win[l] += 1;
                }
                ('G', 'P') | ('C', 'G') | ('P', 'C') => {
                    win[r] += 1;
                }
                _ => {
                    //
                }
            }
        }
        ord.sort_by_key(|&j| (Reverse(win[j]), j));
    }
    for i in 0..(n * 2) {
        println!("{}", ord[i] + 1);
    }
}
