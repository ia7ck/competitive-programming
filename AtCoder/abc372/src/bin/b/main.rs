use proconio::input;

fn main() {
    input! {
        mut m: u64,
    };

    let mut a = Vec::new();
    while m > 0 {
        let p = (0..=10)
            .map_while(|p| match 3_u64.checked_pow(p) {
                Some(x) if x <= m => Some(p),
                _ => None,
            })
            .last()
            .unwrap();
        a.push(p);
        m -= 3_u64.pow(p);
    }

    println!("{}", a.len());
    for i in 0..a.len() {
        print!("{}", a[i]);
        if i + 1 < a.len() {
            print!(" ");
        } else {
            print!("\n");
        }
    }
}
