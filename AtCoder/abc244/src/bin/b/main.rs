use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let _n = scan!(usize);
    let t = scan!(String);

    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;
    for ch in t.chars() {
        match ch {
            'S' => match dir {
                0 => {
                    x += 1;
                }
                1 => {
                    y -= 1;
                }
                2 => {
                    x -= 1;
                }
                3 => {
                    y += 1;
                }
                _ => unreachable!(),
            },
            'R' => {
                dir += 1;
                dir %= 4;
            }
            _ => unreachable!(),
        }
    }
    println!("{} {}", x, y);
}
