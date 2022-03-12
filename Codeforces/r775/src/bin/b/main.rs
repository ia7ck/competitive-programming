use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let t = scan!(usize);
    for _ in 0..t {
        let n = scan!(usize);
        let a = scan!(u64; n);
        solve(n, a);
    }
}

fn solve(_n: usize, a: Vec<u64>) {
    let zero = a.iter().all(|&x| x == 0);
    if zero {
        println!("0");
        return;
    }

    let max_a = a.iter().max().copied().unwrap();
    let sum_a = a.iter().sum::<u64>();
    if sum_a - max_a >= max_a {
        println!("1");
        return;
    }

    let ans = 1 + (max_a - (sum_a - max_a) - 1);
    println!("{}", ans);
}

// 2 5 2
// 1 -> 2 -> 1 -> 2 -> 3 -> 2 -> 3 -> 2 -> ? -> 2 -> ?
// x

// 2 4 2
// 1 -> 2 -> 1 -> 2 -> 3 -> 2 -> 3 -> 2 -> ?

// 2 3 2
// 1 -> 2 -> 1 -> 2 -> 3 -> 2 -> 3 -> ?

// 2 4 4 2
// 1 -> 2 -> 1 -> 2 -> 3 -> 2 -> 3 -> 2 -> 3 -> 4 -> 3 -> 4 -> ?

// 1 5 2
// 2 -> 1 -> 2 -> 3 -> 2 -> 3 -> 2 -> ?
