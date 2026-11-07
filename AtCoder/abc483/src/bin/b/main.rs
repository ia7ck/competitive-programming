use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        k: [usize; q],
    };

    let mut a = (1..=n as i32).collect::<Vec<_>>();
    for k in k {
        let mut back = a.split_off(k);
        a.reverse();
        for i in 0..k {
            a[i] *= -1;
        }
        a.append(&mut back);
    }

    for a in a {
        println!("{}", a);
    }
}
