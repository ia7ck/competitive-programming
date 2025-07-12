use proconio::input;

fn main() {
    input! {
        a: usize,
        n: usize,
    };

    let mut ans = Vec::new();
    for x in 1.. {
        if x >= 1_000_000 {
            break;
        }

        let d = digits(x, 10);
        let rev_d = d.iter().copied().rev().collect::<Vec<_>>();

        let p1 = {
            let mut p = d.clone();
            p.extend(&rev_d);
            p
        };
        let p2 = {
            let mut p = d.clone();
            for i in 1..rev_d.len() {
                p.push(rev_d[i]);
            }
            p
        };

        for p in [p1, p2] {
            assert!(is_palindromic(&p));
            if p[0] != 0 {
                let y = number(&p, 10);
                if y > n {
                    continue;
                }
                let d = digits(y, a);
                if d[0] != 0 && is_palindromic(&d) {
                    ans.push(y);
                }
            }
        }
    }

    ans.sort_unstable();
    ans.dedup();
    let ans = ans.iter().sum::<usize>();
    println!("{}", ans);
}

fn digits(x: usize, a: usize) -> Vec<usize> {
    assert!(2 <= a && a <= 10);
    let mut res = Vec::new();
    let mut x = x;
    while x > 0 {
        res.push(x % a);
        x /= a;
    }
    res.reverse();
    res
}

fn number(d: &Vec<usize>, a: usize) -> usize {
    let mut res = 0;
    for &d in d {
        res = res * a + d;
    }
    res
}

fn is_palindromic(d: &Vec<usize>) -> bool {
    for i in 0..(d.len() / 2) {
        if d[i] != d[d.len() - i - 1] {
            return false;
        }
    }
    true
}
