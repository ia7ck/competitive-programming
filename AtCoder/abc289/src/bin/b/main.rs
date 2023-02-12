use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    let mut ans = Vec::new();
    let mut ord = vec![1];
    for x in 1..n {
        if a.contains(&x) {
            ord.push(x + 1);
        } else {
            ord.reverse();
            ans.append(&mut ord);
            ord.push(x + 1);
        }
    }
    ord.reverse();
    ans.append(&mut ord);
    for i in 0..n {
        print!("{}", ans[i]);
        if i + 1 < n {
            print!(" ");
        } else {
            println!();
        }
    }
}
