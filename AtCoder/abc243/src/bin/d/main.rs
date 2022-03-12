use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (_n, x) = scan!((u64, u64));
    let s = scan!(String);

    let mut stack = Vec::new();
    for ch in s.chars() {
        match ch {
            'U' => {
                if let Some(last) = stack.pop() {
                    if last == 'U' {
                        stack.push(last);
                        stack.push(ch);
                    }
                } else {
                    stack.push(ch);
                }
            }
            'L' => {
                stack.push(ch);
            }
            'R' => {
                stack.push(ch);
            }
            _ => unreachable!(),
        }
    }
    // UU...UULLRLRR...
    let mut x = x;
    for ch in stack {
        match ch {
            'U' => {
                x /= 2;
            }
            'L' => {
                x *= 2;
            }
            'R' => {
                x = x * 2 + 1;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", x);
}

//                         1
//            2                           3
//      4            5             6             7
//   8     9     10     11     12     13     14     15
// 16 17 18 19 20  21 22  23 24  25 26  27 28  29 30  31
