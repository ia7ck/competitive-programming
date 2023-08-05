use std::io::BufReader;

use proconio::{input, source::line::LineSource};

fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(BufReader::new(stdin));
    input! {
        from &mut source,
        n: usize,
        k: usize,
    };
    let mut counter = 0;

    let mut ask = |x: Vec<usize>| {
        counter += 1;
        assert!(counter <= n);
        // eprintln!("x = {:?}", x);
        assert_eq!(x.len(), k);
        for &x in &x {
            assert!(1 <= x && x <= n);
        }
        print!("?");
        for x in x {
            print!(" {}", x);
        }
        println!();
        input! {
            from &mut source,
            t: u8,
        };
        // eprintln!("t = {}", t);
        t
    };

    // N = 5, K = 3

    // . 2 3 4
    // 1 . 3 4
    // 1 2 . 4

    let mut resp = vec![0; k];
    for i in 1..=k {
        let mut x = Vec::new();
        for j in 1..=(k + 1) {
            if i != j {
                x.push(j);
            }
        }
        resp[i - 1] = ask(x);
    }

    let mut same = 0;
    for i in 1..k {
        if resp[i] == resp[0] {
            same += 1;
        }
    }

    // 先頭を特定する
    let mut a = vec![0; n];
    let prefix = ask((1..=k).collect());
    a[0] = match (prefix, same % 2) {
        (0, 0) => 0,
        (0, 1) => 1,
        (1, 0) => 1,
        (1, 1) => 0,
        _ => unreachable!(),
    };

    for i in 1..k {
        if resp[i] == resp[0] {
            a[i] = a[0];
        } else {
            a[i] = 1 - a[0];
        }
    }

    a[k] = match (a[1..k].iter().sum::<usize>() % 2, resp[0]) {
        (0, 0) => 0,
        (0, 1) => 1,
        (1, 0) => 1,
        (1, 1) => 0,
        _ => unreachable!(),
    };

    // ここまで K+1 回の質問
    // 先頭 K+1 項を特定

    // . . 3 4 5

    for i in (k + 2)..=n {
        let mut x = Vec::new();
        let mut sum = 0;
        for j in (i - k + 1)..=i {
            x.push(j);
            if j < i {
                sum += a[j - 1];
            }
        }
        a[i - 1] = match (sum % 2, ask(x)) {
            (0, 0) => 0,
            (0, 1) => 1,
            (1, 0) => 1,
            (1, 1) => 0,
            _ => unreachable!(),
        };
    }

    print!("!");
    for a in a {
        print!(" {}", a);
    }
    println!();
}
