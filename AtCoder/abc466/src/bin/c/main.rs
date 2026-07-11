use proconio::input_interactive;

fn main() {
    input_interactive! {
        n: usize,
    };

    let query = |i: usize, j: usize| -> bool {
        assert!(j < n);
        assert!(i < j);

        println!("? {} {}", i + 1, j + 1);

        input_interactive! {
            resp: String,
        };

        resp == "Yes"
    };

    let mut ans = 0;
    let mut i = 0;
    for j in 1..n {
        while i < j {
            if query(i, j) {
                break;
            } else {
                i += 1;
            }
        }

        // i, i + 1, ..., j - 1
        ans += j - i;
    }

    println!("! {}", ans);
}
