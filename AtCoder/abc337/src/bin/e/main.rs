use proconio::{input_interactive, marker::Bytes};

fn main() {
    input_interactive!{
        n: usize,
    };

    let m = n.next_power_of_two().ilog2() as usize;
    let mut juice = vec![vec![]; m];
    for i in 0..n {
        for j in 0..m {
            if i >> j & 1 == 1 {
                juice[j].push(i);
            }
        }
    }
    println!("{}", m);
    for i in 0..m {
        print!("{}", juice[i].len());
        for j in 0..juice[i].len() {
            print!(" {}", juice[i][j] + 1);
        }
        println!();
    }

    input_interactive!{
        s: Bytes
    };

    let mut ans = Vec::new();
    for i in 0..n {
        let mut ok = true;
        for j in 0..m {
            if s[j] == b'1' {
                if juice[j].contains(&i) == false {
                    ok = false;
                }
            } else {
                if juice[j].contains(&i) {
                    ok = false;
                }
            }
        }
        if ok {
            ans.push(i);
        }
    }
    assert_eq!(ans.len(), 1);
    println!("{}", ans[0] + 1);
}
