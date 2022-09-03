use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };

    #[rustfmt::skip]
    let pins = vec![
        vec![],
        vec![7],
        vec![4],
        vec![2, 8],
        vec![1, 5],
        vec![3, 9],
        vec![6],
        vec![10],
    ];

    if s[0] == '1' {
        println!("No");
        return;
    }

    for i in 1..=7 {
        for j in (i + 2)..=7 {
            let ok_i = pins[i].iter().any(|&p| s[p - 1] == '1');
            let ok_j = pins[j].iter().any(|&p| s[p - 1] == '1');
            if ok_i && ok_j {
                for k in (i + 1)..j {
                    let ok = pins[k].iter().all(|&p| s[p - 1] == '0');
                    if ok {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
