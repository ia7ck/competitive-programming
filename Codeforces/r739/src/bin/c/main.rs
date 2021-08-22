use procon_reader::ProconReader;
use std::cmp::Ordering;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let t: usize = rd.get();
    for _ in 0..t {
        let k: u64 = rd.get();
        solve(k);
    }
}

fn solve(k: u64) {
    let mut diag = 1;
    for x in 1.. {
        if diag - (x - 1) <= k && k <= diag + (x - 1) {
            match diag.cmp(&k) {
                Ordering::Less => {
                    println!("{} {}", x, x - (k - diag));
                }
                Ordering::Equal => {
                    println!("{} {}", x, x);
                }
                Ordering::Greater => {
                    println!("{} {}", x - (diag - k), x);
                }
            }
            break;
        }
        diag += x * 2;
    }
}

// 1,  3,  7,  13,  21,  31
//   2   4   6    8    10
