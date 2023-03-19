use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    };

    let mut c = Vec::new();
    for i in 0..n {
        c.push((a[i], i, 'A'));
    }
    for j in 0..m {
        c.push((b[j], j, 'B'));
    }
    c.sort();

    let mut ans_a = vec![0; n];
    let mut ans_b = vec![0; m];
    for (i, &(_, k, c)) in c.iter().enumerate() {
        if c == 'A' {
            ans_a[k] = i + 1;
        } else {
            ans_b[k] = i + 1;
        }
    }
    for i in 0..n {
        print!("{}", ans_a[i]);
        if i + 1 < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
    for j in 0..m {
        print!("{}", ans_b[j]);
        if j + 1 < m {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
