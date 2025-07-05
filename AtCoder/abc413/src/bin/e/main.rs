use proconio::input;

fn main() {
    input! {
        t: usize,
    };

    for _ in 0..t {
        input! {
            n: u32,
            a: [usize; 2_usize.pow(n)],
        };

        solve(n, a);
    }
}

fn solve(_n: u32, a: Vec<usize>) {
    let ans = f(a);
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn f(a: Vec<usize>) -> Vec<usize> {
    assert!(!a.is_empty());
    if a.len() == 1 {
        return a;
    }

    let min = a.iter().min().copied().unwrap();
    let mut a = a;
    if a.iter().position(|&x| x == min).unwrap() < a.len() / 2 {
        //
    } else {
        a.reverse();
    }

    let mut a = a;
    let b = a.split_off(a.len() / 2);

    a = f(a);
    a.append(&mut f(b));
    a
}
