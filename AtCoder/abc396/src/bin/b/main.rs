use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut stack = vec![0; 100];

    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                x: u32,
            };
            stack.push(x);
        } else {
            let ans = stack.pop().unwrap();
            println!("{}", ans);
        }
    }
}
