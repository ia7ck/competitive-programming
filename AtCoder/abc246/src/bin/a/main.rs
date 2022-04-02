use scanner_proc_macro::insert_scanner;

#[insert_scanner]
fn main() {
    let (mut x1, mut y1) = scan!((i32, i32));
    let (mut x2, mut y2) = scan!((i32, i32));
    let (mut x3, mut y3) = scan!((i32, i32));

    if x1 != x2 {
        if x2 == x3 {
            std::mem::swap(&mut x1, &mut x3);
            std::mem::swap(&mut y1, &mut y3);
        } else {
            std::mem::swap(&mut x2, &mut x3);
            std::mem::swap(&mut y2, &mut y3);
        }
    }

    assert_eq!(x1, x2);
    assert_ne!(y1, y2);

    let y = if y1 == y3 { y2 } else { y1 };
    println!("{} {}", x3, y);
}
