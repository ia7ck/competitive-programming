use proconio::input;

fn main() {
    input! {
        n: u32,
        m: usize,
        a: [u32; m],
    };

    let mut ans = Vec::new();
    for i in 1..=n {
        if !a.contains(&i) {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
