fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let t: Vec<char> = s.trim().chars().collect();
    for (i, c) in t.iter().enumerate() {
        if i % 2 == 0 {
            if (*c as u8) < 'a' as u8 || (*c as u8) > 'z' as u8 {
                println!("No");
                return;
            }
        } else {
            if *c != ' ' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
