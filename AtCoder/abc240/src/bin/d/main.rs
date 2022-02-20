use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let n = scan!(usize);
    let a = scan!(usize; n);

    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut ans = Vec::new();
    for i in 0..n {
        if let Some(&(val, len)) = stack.last() {
            if val == a[i] {
                if len + 1 == a[i] {
                    for l in 0..len {
                        let b = stack.pop();
                        assert_eq!(b, Some((a[i], len - l)));
                    }
                } else {
                    stack.push((a[i], len + 1));
                }
            } else {
                stack.push((a[i], 1));
            }
        } else {
            stack.push((a[i], 1));
        }
        ans.push(stack.len());
    }
    for ans in ans {
        println!("{}", ans);
    }
}
