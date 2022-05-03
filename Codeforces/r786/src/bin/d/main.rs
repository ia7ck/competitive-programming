use binary_search_range::BinarySearchRange;
use proconio::input;

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            a: [u32; n],
        };
        solve(n, a);
    }
}

fn solve(n: usize, a: Vec<u32>) {
    let mut b = a.clone();
    b.sort();

    if n % 2 == 0 {
        for i in 0..(n / 2) {
            let rng1 = b.range(a[i * 2]..(a[i * 2] + 1));
            let rng2 = b.range(a[i * 2 + 1]..(a[i * 2 + 1] + 1));
            if (rng1.contains(&(i * 2)) && rng2.contains(&(i * 2 + 1)))
                || (rng1.contains(&(i * 2 + 1)) && rng2.contains(&(i * 2)))
            {
                //
            } else {
                println!("NO");
                return;
            }
        }
        println!("YES");
    } else {
        if a[0] != b[0] {
            println!("NO");
            return;
        }

        for i in 0..(n / 2) {
            let rng1 = b.range(a[i * 2 + 1]..(a[i * 2 + 1] + 1));
            let rng2 = b.range(a[i * 2 + 2]..(a[i * 2 + 2] + 1));
            if (rng1.contains(&(i * 2 + 1)) && rng2.contains(&(i * 2 + 2)))
                || (rng1.contains(&(i * 2 + 2)) && rng2.contains(&(i * 2 + 1)))
            {
                //
            } else {
                println!("NO");
                return;
            }
        }
        println!("YES");
    }
}
