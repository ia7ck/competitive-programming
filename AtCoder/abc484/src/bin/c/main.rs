use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        b: [u32; n],
    };

    if a == b {
        println!("Yes");
        return;
    }

    let l = (0..n).find(|&i| a[i] != b[i]).unwrap();
    let r = (0..n).rfind(|&i| a[i] != b[i]).unwrap();

    let a = a[l..=r].to_vec();
    let b = b[l..=r].to_vec();

    let al = {
        let mut a = a.clone();
        a.rotate_left(1);
        a
    };
    let ar = {
        let mut a = a.clone();
        a.rotate_right(1);
        a
    };

    if al == b || ar == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
