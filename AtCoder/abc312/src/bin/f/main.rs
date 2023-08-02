use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..n {
        input! {
            t: u8,
        };
        if t == 0 {
            input! {
                aa: u64,
            };
            a.push(aa);
        } else if t == 1 {
            input! {
                bb: u64,
            };
            b.push(bb);
        } else {
            input! {
                cc: usize,
            };
            c.push(cc);
        }
    }

    a.sort();
    a.reverse();
    let mut cul_a = vec![0; m + 1];
    for i in 0..m {
        if i < a.len() {
            cul_a[i + 1] = cul_a[i] + a[i];
        } else {
            cul_a[i + 1] = cul_a[i];
        }
    }

    b.sort();
    c.sort();
    let mut ans = cul_a[cul_a.len() - 1];
    let mut o = 0;
    let mut sum_b = 0;
    for i in 0..m {
        if o == 0 {
            if let Some(c) = c.pop() {
                o += c;
            } else {
                break;
            }
        } else {
            if let Some(b) = b.pop() {
                sum_b += b;
                o -= 1;
            } else {
                break;
            }
        }
        // eprintln!("i = {}, o = {}, sum_b = {}", i, o, sum_b);
        ans = ans.max(sum_b + cul_a[m - i - 1]);
    }
    println!("{}", ans);
}
