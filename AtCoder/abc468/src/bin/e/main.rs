use ac_library::{ModInt998244353, Monoid, Segtree};
use proconio::input;

fn main() {
    if false {
        let n = 2;
        for i in 1..=n {
            let mut lens = vec![0; n + 1];
            for l in 1..=n {
                for r in l..=n {
                    if (l..=r).contains(&i) {
                        lens[r - l + 1] += 1;
                    }
                }
            }
            eprintln!("i = {i}, len = {lens:?}");
        }
    }

    input! {
        n: usize,
        a: [usize; n],
    };

    type Mint = ModInt998244353;
    #[derive(Clone)]
    struct S(Mint);
    impl Monoid for S {
        type S = S;

        fn identity() -> Self::S {
            S(Mint::new(0))
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            S(a.0 + b.0)
        }
    }

    let mut seg = Segtree::<S>::new(n + 1);
    for i in 1..=n {
        seg.set(i, S(Mint::new(1) / i));
    }

    let a = {
        let mut a = a;
        a.insert(0, 0);
        a
    };
    let mut ans = Mint::new(0);
    let mut c = seg.all_prod().0;
    for i in 1..=(n / 2) {
        ans += c * a[i];
        ans += c * a[n - i + 1];

        let add = seg.prod((i + 1)..=(n - i)).0;
        c += add;
    }
    if n % 2 == 1 {
        ans += c * a[(n + 1) / 2];
    }

    println!("{}", ans.val());
}

// n = 2
// i = 1, len = [0, 1, 1]
// i = 2, len = [0, 1, 1]

// n = 7
// i = 1, len = [0, 1, 1, 1, 1, 1, 1, 1]
// i = 2, len = [0, 1, 2, 2, 2, 2, 2, 1]
// i = 3, len = [0, 1, 2, 3, 3, 3, 2, 1]
// i = 4, len = [0, 1, 2, 3, 4, 3, 2, 1]
// i = 5, len = [0, 1, 2, 3, 3, 3, 2, 1]
// i = 6, len = [0, 1, 2, 2, 2, 2, 2, 1]
// i = 7, len = [0, 1, 1, 1, 1, 1, 1, 1]

// n = 8
// i = 1, len = [0, 1, 1, 1, 1, 1, 1, 1, 1]
// i = 2, len = [0, 1, 2, 2, 2, 2, 2, 2, 1]
// i = 3, len = [0, 1, 2, 3, 3, 3, 3, 2, 1]
// i = 4, len = [0, 1, 2, 3, 4, 4, 3, 2, 1]
// i = 5, len = [0, 1, 2, 3, 4, 4, 3, 2, 1]
// i = 6, len = [0, 1, 2, 3, 3, 3, 3, 2, 1]
// i = 7, len = [0, 1, 2, 2, 2, 2, 2, 2, 1]
// i = 8, len = [0, 1, 1, 1, 1, 1, 1, 1, 1]
