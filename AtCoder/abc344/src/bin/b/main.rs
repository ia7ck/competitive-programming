use proconio::input;

fn main() {
    let mut ans = Vec::new();
    loop {
        input! {
            a: u32,
        };
        ans.push(a);
        if a == 0 {
            println!(
                "{}",
                ans.iter()
                    .rev()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            break;
        }
    }
}
