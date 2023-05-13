use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    let mut b = vec![a[0]];
    for i in 1..n {
        if a[i - 1] < a[i] {
            b.extend((a[i - 1] + 1)..a[i]);
        } else {
            b.extend(((a[i] + 1)..a[i - 1]).rev());
        }
        b.push(a[i]);
    }

    for i in 0..b.len() {
        print!("{}", b[i]);
        if i + 1 < b.len() {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
