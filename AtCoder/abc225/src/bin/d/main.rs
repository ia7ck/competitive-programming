use input_i_scanner::{scan_with, InputIScanner};
use join::Join;

fn main() {
    let stdin = std::io::stdin();
    let mut _i_i = InputIScanner::from(stdin.lock());

    let (n, q) = scan_with!(_i_i, (usize, usize));

    let mut pre: Vec<usize> = (0..=n).collect();
    let mut succ: Vec<usize> = (0..=n).collect();
    for _ in 0..q {
        let t = scan_with!(_i_i, u8);
        match t {
            1 => {
                let (x, y) = scan_with!(_i_i, (usize, usize));
                assert_eq!(pre[y], y);
                pre[y] = x;
                assert_eq!(succ[x], x);
                succ[x] = y;
            }
            2 => {
                let (x, y) = scan_with!(_i_i, (usize, usize));
                assert_eq!(pre[y], x);
                pre[y] = y;
                assert_eq!(succ[x], y);
                succ[x] = x;
            }
            3 => {
                let mut x = scan_with!(_i_i, usize);
                while pre[x] != x {
                    x = pre[x];
                }
                let mut ans = Vec::new();
                ans.push(x);
                while succ[x] != x {
                    x = succ[x];
                    ans.push(x);
                }
                print!("{} ", ans.len());
                println!("{}", ans.iter().join(" "));
            }
            _ => unreachable!(),
        }
    }
}
