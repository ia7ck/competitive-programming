use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        mut p: [Usize1; n],
    };

    let mut ans = Vec::new();

    macro_rules! swap_a {
        ($i: expr) => {
            assert!($i + 1 < n);
            p.swap($i, $i + 1);
            ans.push(('A', $i));
        };
    }

    macro_rules! swap_b {
        ($i: expr) => {
            assert!($i + 2 < n);
            p.swap($i, $i + 2);
            ans.push(('B', $i));
        };
    }

    for i in 0..n {
        while i % 2 != p[i] % 2 {
            let mut j = i;
            for k in ((i + 1)..n).step_by(2) {
                if k % 2 != p[k] % 2 {
                    j = k;
                    break;
                }
            }
            assert_ne!(j, i);
            for k in (i..(j - 1)).step_by(2) {
                swap_b!(k);
            }
            assert_ne!((j - 1) % 2, p[j - 1] % 2);
            swap_a!(j - 1);
        }
    }

    for start in 0..=1 {
        for i in (start..n).step_by(2) {
            if i != p[i] {
                let mut j = i;
                for k in ((i + 2)..n).step_by(2) {
                    if i == p[k] {
                        j = k;
                        break;
                    }
                }
                assert_ne!(j, i);
                for k in (i..(j - 1)).step_by(2).rev() {
                    swap_b!(k);
                }
            }
        }
    }

    for w in p.windows(2) {
        assert!(w[0] < w[1]);
    }

    assert!(ans.len() <= 100000);
    println!("{}", ans.len());
    for (t, i) in ans {
        println!("{} {}", t, i + 1);
    }
}
