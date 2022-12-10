use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [u64; n],
    };

    let mut set = BTreeSet::<(u64, usize)>::new();
    for i in 0..m {
        set.insert((a[i], i));
    }
    let (mut y, mut j) = set.iter().nth(k - 1).copied().unwrap();
    let mut sum = set.iter().take(k).map(|(x, _)| x).sum::<u64>();
    let mut ans = vec![sum];
    for i in m..n {
        set.insert((a[i], i));
        if a[i] < y {
            // sum += a[i] - y
            sum += a[i];
            assert!(sum >= y);
            sum -= y;
            let (ny, nj) = set.range((0, 0)..(y, j)).last().copied().unwrap();
            y = ny;
            j = nj;
        }
        if a[i - m] <= y {
            // sum += -a[i - m] + ny
            let (ny, nj) = set.range((y, j)..(std::u64::MAX, std::usize::MAX)).nth(1).copied().unwrap();
            sum += ny;
            assert!(sum >= a[i - m]);
            sum -= a[i - m];
            y = ny;
            j = nj;
        }
        set.remove(&(a[i - m], i - m));
        // eprintln!("y = {}, set = {:?}", y, set);
        ans.push(sum);
    }

    for i in 0..ans.len() {
        print!("{}", ans[i]);
        if i + 1 < ans.len() {
            print!(" ");
        } else {
            println!();
        }
    }
}

// 10 6 3
// 12 2 17 11 19 8 4 3 6 20
// 12 2 17 11 19 8               2 8 11 12 17 19
//    2 17 11 19 8 4             2 4  8 11 17 19
//      17 11 19 8 4 3           3 4  8 11 17 19 
//         11 19 8 4 3 6         3 4  6  8 11 19
//            19 8 4 3 6 20      3 4  6  8 19 20
