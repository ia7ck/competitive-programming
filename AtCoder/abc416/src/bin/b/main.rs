use proconio::input;

fn main() {
    input! {
        s: String,
    };
    
    let mut ans = Vec::new();
    for x in s.split('#') {
        let mut x = x.chars().collect::<Vec<_>>();
        if !x.is_empty() {
            x[0] = 'o';
        }
        ans.push(x.iter().collect::<String>());
    }

    println!("{}", ans.join("#"));
}
