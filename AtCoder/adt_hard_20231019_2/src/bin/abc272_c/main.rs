use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut odd = a
        .iter()
        .copied()
        .filter(|&x| x % 2 == 1)
        .collect::<Vec<_>>();
    let mut even = a
        .iter()
        .copied()
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<_>>();
    odd.sort();
    odd.reverse();
    even.sort();
    even.reverse();

    let mut ans = None;
    if odd.len() >= 2 {
        let x = odd[0] + odd[1];
        ans = match ans.take() {
            Some(y) => Some(x.max(y)),
            None => Some(x),
        };
    }
    if even.len() >= 2 {
        let x = even[0] + even[1];
        ans = match ans.take() {
            Some(y) => Some(x.max(y)),
            None => Some(x),
        };
    }
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
