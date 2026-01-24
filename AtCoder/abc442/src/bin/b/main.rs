use proconio::input;

fn main() {
    input! {
        q: usize,
        queries: [u8; q],
    };

    let mut volume = 0;
    let mut playing = false;
    for a in queries {
        if a == 1 {
            volume += 1;
        } else if a == 2 {
            if volume >= 1 {
                volume -= 1;
            }
        } else {
            playing = !playing;
        }

        let ans = volume >= 3 && playing;

        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
