use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut odd = a.iter().copied().filter(|x| x % 2 == 1).collect::<Vec<_>>();
    let mut even = a.iter().copied().filter(|x| x % 2 == 0).collect::<Vec<_>>();
    odd.sort();
    odd.reverse();
    even.sort();
    even.reverse();
    let mut ans = -1;
    if let Some(&x) = odd.get(0) {
        if let Some(&y) = odd.get(1) {
            ans = ans.max(x + y);
        }
    }
    if let Some(&x) = even.get(0) {
        if let Some(&y) = even.get(1) {
            ans = ans.max(x + y);
        }
    }
    println!("{}", ans);
}
