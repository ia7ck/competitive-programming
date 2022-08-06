use proconio::input;

fn main() {
    input! {
        mut a: [u8; 5],
    };

    a.sort();
    if (a[0] == a[1] && a[2] == a[3] && a[3] == a[4])
        || (a[0] == a[1] && a[1] == a[2] && a[3] == a[4])
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
