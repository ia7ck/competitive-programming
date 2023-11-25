use proconio::input;

fn main() {
    input! {
        n: usize,
        l: u32,
        r: u32,
        a: [u32; n],
    };

    let mut ans = Vec::new();
    for i in 0..n {
        if l <= a[i] && a[i] <= r {
            ans.push(a[i]);
        } else if a[i] < l {
            ans.push(l);
        } else {
            ans.push(r);
        }
    }
    for i in 0..n {
        print!("{}", ans[i]);
        if i + 1 < n {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
