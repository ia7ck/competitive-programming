use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut ans = vec![vec!['.'; n]; n];
    for i in 0..n {
        let j = n - i - 1;
        if i <= j {
            // (i, i), (j, j)
            for y in i..=j {
                for x in i..=j {
                    if i % 2 == 0 {
                        ans[y][x] = '#';
                    } else {
                        ans[y][x] = '.';
                    }
                }
            }
        }
    }

    for row in ans {
        println!("{}", row.iter().collect::<String>());
    }
}
