use input_i_scanner::InputIScanner;
use join::Join;

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

    let (a, b) = scan!((String, String));
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let mut swap = false;
    let (a, b) = if a.len() <= b.len() {
        (a, b)
    } else {
        swap = true;
        (b, a)
    };
    let mut freq_a = [0usize; 10];
    let mut freq_b = [0usize; 10];
    for &ch in &a {
        freq_a[ch as usize - '0' as usize] += 1;
    }
    for &ch in &b {
        freq_b[ch as usize - '0' as usize] += 1;
    }
    let mut ans = std::usize::MAX;
    let mut ans_a = Vec::new();
    let mut ans_b = Vec::new();
    for d1 in 1..=9 {
        for d2 in 1..=9 {
            if freq_a[d1] == 0 || freq_b[d2] == 0 {
                continue;
            }
            let mut f_a = freq_a.clone();
            let mut f_b = freq_b.clone();
            f_a[d1] -= 1;
            f_b[d2] -= 1;
            let mut a = vec![d1];
            let mut b = vec![d2];
            for d in 1..=9 {
                let count = f_a[d].min(f_b[9 - d]);
                for _ in 0..count {
                    a.push(d);
                    b.push(9 - d);
                }
                f_a[d] -= count;
                f_b[9 - d] -= count;
            }
            for d in 1..=9 {
                for dd in (9-d)..=9 {
                    let count = f_a[d].min(f_b[dd]);
                    for _ in 0..count {
                        a.push(d);
                        b.push(dd);
                    }
                    f_a[d] -= count;
                    f_b[dd] -= count;
                }
            }
            for d in (1..=9).rev() {
                for _ in 0..f_a[d] {
                    a.push(d);
                }
                for _ in 0..f_b[d] {
                    b.push(d);
                }
            }
            a.reverse();
            b.reverse();
            // eprintln!("a = {:?}", a);
            // eprintln!("b = {:?}", b);
            // eprintln!();
            let digit_sum = digit_sum_of_sum(&a, &b);
            if ans > digit_sum {
                ans = digit_sum;
                ans_a = a;
                ans_b = b;
            }
        }
    }
    assert_ne!(ans, std::usize::MAX);
    eprintln!("{}", ans);
    if swap {
        println!("{}", ans_b.iter().join(""));
        println!("{}", ans_a.iter().join(""));
    } else {
        println!("{}", ans_a.iter().join(""));
        println!("{}", ans_b.iter().join(""));
    }
}

fn digit_sum_of_sum(a: &[usize], b: &[usize]) -> usize {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.reverse();
    b.reverse();
    let mut carry = 0;
    let mut sum = Vec::new();
    for i in 0..(a.len().max(b.len())) {
        let x = if i < a.len() && i < b.len() {
            a[i] + b[i] + carry
        } else if i < a.len() {
            a[i] + carry
        } else if i < b.len() {
            b[i] + carry
        } else {
            unreachable!()
        };
        if x < 10 {
            sum.push(x);
            carry = 0;
        } else {
            sum.push(x % 10);
            carry = 1;
        }
    }
    if carry == 1 {
        sum.push(carry);
    }
    sum.reverse();
    sum.iter().sum::<usize>()
}
