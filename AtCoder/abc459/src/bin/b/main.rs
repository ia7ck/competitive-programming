use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut ans = Vec::new();
    for s in s {
        let c = match s[0] {
            'a' | 'b' | 'c' => 2,
            'd' | 'e' | 'f' => 3,
            'g' | 'h' | 'i' => 4,
            'j' | 'k' | 'l' => 5,
            'm' | 'n' | 'o' => 6,
            'p' | 'q' | 'r' | 's' => 7,
            't' | 'u' | 'v' => 8,
            'w' | 'x' | 'y' | 'z' => 9,
            _ => unreachable!(),
        };
        ans.push(c);
    }

    for c in ans {
        print!("{}", c);
    }
    println!();
}
