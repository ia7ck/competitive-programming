use proconio::input;

fn main() {
    input! {
        q: usize,
    };

    let mut cum_sum = Vec::<isize>::new();
    let mut neg = 0; // cum_sum[*] < 0
    for _ in 0..q {
        input! {
            op: u8,
        };

        if op == 1 {
            input! {
                c: char,
            };
            let last = cum_sum.last().unwrap_or(&0);
            let new = if c == '(' { *last + 1 } else { *last - 1 };
            cum_sum.push(new);
            if new < 0 {
                neg += 1;
            }
        } else {
            let last = cum_sum.pop().unwrap();
            if last < 0 {
                neg -= 1;
            }
        }

        if neg == 0 && (cum_sum.is_empty() || cum_sum.last() == Some(&0)) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
