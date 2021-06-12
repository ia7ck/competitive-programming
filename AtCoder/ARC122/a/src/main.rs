use mod_int::ModInt1000000007;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let a: Vec<u64> = rd.get_vec(n);

    if n == 1 {
        println!("{}", a[0]);
        return;
    }

    type Mint = ModInt1000000007;
    let mut fib = vec![Mint::new(0), Mint::new(1)];
    for i in 1..=n {
        let x = fib[i - 1] + fib[i];
        fib.push(x);
    }
    use std::iter;
    let idx: Vec<usize> = if n % 2 == 0 {
        let it = (1..(n - 2)).step_by(2);
        iter::once(n - 1)
            .chain(it.clone().rev())
            .chain(it)
            .collect()
    } else {
        let it = (2..(n - 2)).step_by(2);
        iter::once(n - 1)
            .chain(it.clone().rev())
            .chain(iter::once(0))
            .chain(it)
            .collect()
    };
    let mut positive = fib[n + 1];
    let mut ans = Mint::new(a[0]) * positive;
    let mut neg = true;
    for (i, &j) in idx.iter().enumerate() {
        if neg {
            positive = positive - fib[j];
        } else {
            positive = positive + fib[j];
        }
        let x = Mint::new(a[i + 1]) * positive - Mint::new(a[i + 1]) * (fib[n + 1] - positive);
        ans = ans + x;
        if j != 0 {
            neg = !neg;
        }
    }
    println!("{}", ans.val());
}

// 3
// [3, 2, 2]

// 4
// 5 3 4 3

// 5
// 8 5 6 6 5
// 0 3 2 2 3

// 6
// 13 8 10 9 10 8
// 0  5  3 4  3 5

// 7
// [21, 13, 16, 15, 15, 16, 13]
