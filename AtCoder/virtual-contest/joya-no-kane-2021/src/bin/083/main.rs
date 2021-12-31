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

    let mut p: Vec<usize> = p.into_iter().map(|p| p - 1).collect();
    let mut position = vec![0; n];
    for (i, &p) in p.iter().enumerate() {
        position[p] = i;
    }
    let mut seen = vec![false; n];
    let mut ans = Vec::new();
    for i in 0..n {
        if p[i] == i {
            continue;
        }
        let j = position[i];
        assert!(i < j);
        for k in (i..j).rev() {
            position.swap(p[k], p[k + 1]);
            p.swap(k, k + 1);
            if seen[k] {
                println!("-1");
                return;
            }
            seen[k] = true;
            ans.push(k);
        }
        assert_eq!(p[i], i);
    }
    if ans.len() != n - 1 {
        println!("-1");
        return;
    }
    for ans in ans {
        println!("{}", ans + 1);
    }
}
