use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n <= 10 {
        println!("{}", n - 1);
        return;
    }

    let l = (0..40)
        .find(|&l| n <= (0..=l).map(|k| count_p(k)).sum::<usize>())
        .unwrap();
    assert!(l >= 1);
    let mut m = (0..l).map(|k| count_p(k)).sum::<usize>();
    let mut ans = Vec::new();
    for i in 0..(l / 2) {
        for d in usize::from(i == 0)..=9 {
            if m + f(l - (i + 1) * 2) < n {
                m += f(l - (i + 1) * 2);
            } else {
                ans.push(d);
                break;
            }
        }
    }

    if l % 2 == 1 {
        assert!(1 <= n - m);
        assert!(n - m <= 10);
        let mut tmp = ans.clone();
        ans.push(n - m - 1);
        tmp.reverse();
        ans.extend(tmp);
    } else {
        let mut tmp = ans.clone();
        tmp.reverse();
        ans.extend(tmp);
    }
    for d in ans {
        print!("{}", d);
    }
    println!();
}

fn count_p(l: usize) -> usize {
    assert!(l <= 40);
    match l {
        0 => 0,
        1 => 10,
        2 => 9,
        l => 9 * 10_usize.pow(((l - 1) / 2) as u32),
    }
}

fn f(l: usize) -> usize {
    match l {
        0 => 1,
        l => 10_usize.pow(((l + 1) / 2) as u32),
    }
}
