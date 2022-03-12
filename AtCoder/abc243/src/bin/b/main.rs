use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let n = scan!(usize);
    let a = scan!(u32; n);
    let b = scan!(u32; n);

    let mut hit = 0;
    for i in 0..n {
        if a[i] == b[i] {
            hit += 1;
        }
    }
    let mut blow = 0;
    for i in 0..n {
        for j in 0..n {
            if i != j && a[i] == b[j] {
                blow += 1;
            }
        }
    }
    println!("{}", hit);
    println!("{}", blow);
}
