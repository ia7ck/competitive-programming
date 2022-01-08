use input_i_scanner::InputIScanner;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    macro_rules! scan {
        (($($t: ty),+)) => {
            ($(scan!($t)),+)
        };
        ($t: ty) => {
            _i_i.scan::<$t>() as $t
        };
        (($($t: ty),+); $n: expr) => {
            std::iter::repeat_with(|| scan!(($($t),+))).take($n).collect::<Vec<_>>()
        };
        ($t: ty; $n: expr) => {
            std::iter::repeat_with(|| scan!($t)).take($n).collect::<Vec<_>>()
        };
    }

    let n = scan!(usize);
    let p = scan!(usize; n);

    if n == 2 {
        if p == vec![1, 2] {
            println!("0");
        } else {
            println!("1");
        }
        return;
    }

    let mut p_sorted = p.clone();
    p_sorted.sort();
    if p == p_sorted {
        println!("0");
        return;
    }

    let (k, increase) = {
        if p[0] + 1 == p[1] {
            let mut k = 1;
            while k + 1 < n {
                if p[k] > p[k + 1] {
                    break;
                }
                k += 1;
            }
            (k + 1, true)
        } else if p[0] == p[1] + 1 {
            let mut k = 1;
            while k + 1 < n {
                if p[k] < p[k + 1] {
                    break;
                }
                k += 1;
            }
            (k + 1, false)
        } else {
            (1, p[1] < p[2])
        }
    };
    assert!(k < n);

    if increase {
        println!("{}", solve1(&p, k));
    } else {
        println!("{}", solve2(&p, k));
    }
}

fn solve1(a: &[usize], k: usize) -> usize {
    for i in 1..k {
        assert_eq!(a[i - 1] + 1, a[i]);
    }
    for i in k..(a.len() - 1) {
        assert_eq!(a[i] + 1, a[i + 1]);
    }
    k.min(1 + (a.len() - k) + 1)
}

fn solve2(a: &[usize], k: usize) -> usize {
    for i in 1..k {
        assert_eq!(a[i - 1], a[i] + 1);
    }
    for i in k..(a.len() - 1) {
        assert_eq!(a[i], a[i + 1] + 1);
    }
    (k + 1).min(1 + (a.len() - k))
}
