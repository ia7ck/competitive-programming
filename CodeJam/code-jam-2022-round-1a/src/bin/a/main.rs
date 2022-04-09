use join::Join;
use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let t = scan!(usize);
    for t in 1..=t {
        print!("Case #{}: ", t);
        let s = scan!(String);
        let s: Vec<char> = s.chars().collect();
        let mut t = Vec::new();
        t.push(s[s.len() - 1]);
        for i in (0..(s.len() - 1)).rev() {
            let c = s[i];
            let d = s[i + 1];
            if c < d {
                t.push(c);
                t.push(c);
            } else if c > d {
                t.push(c);
            } else {
                let mut e = c;
                for j in i..s.len() {
                    if c != s[j] {
                        e = s[j];
                        break;
                    }
                }
                if c < e {
                    t.push(c);
                    t.push(c);
                } else {
                    t.push(c);
                }
            }
        }
        t.reverse();
        println!("{}", t.iter().join(""));
    }
}
