use binary_search::BinarySearch;
use procon_reader::ProconReader;

fn main() {
    let stdin = std::io::stdin();
    let mut rd = ProconReader::new(stdin.lock());

    let n: usize = rd.get();
    let mut red = Vec::new();
    let mut green = Vec::new();
    let mut blue = Vec::new();
    for _ in 0..(n * 2) {
        let a: i64 = rd.get();
        let c: char = rd.get();
        match c {
            'R' => {
                red.push(a);
            }
            'G' => {
                green.push(a);
            }
            'B' => {
                blue.push(a);
            }
            _ => unreachable!(),
        }
    }
    if red.len() % 2 == 0 && green.len() % 2 == 0 && blue.len() % 2 == 0 {
        println!("0");
        return;
    }
    red.sort();
    green.sort();
    blue.sort();
    if red.len() % 2 == 0 {
        solve(red, green, blue);
        return;
    }
    if green.len() % 2 == 0 {
        solve(green, red, blue);
        return;
    }
    if blue.len() % 2 == 0 {
        solve(blue, red, green);
        return;
    }
    unreachable!();
}

fn solve(red: Vec<i64>, green: Vec<i64>, blue: Vec<i64>) {
    assert_eq!(red.len() % 2, 0);
    assert_eq!(green.len() % 2, 1);
    assert_eq!(blue.len() % 2, 1);

    // println!("{:?}", red);
    // println!("{:?}", green);
    // println!("{:?}", blue);

    let mut rg = std::i64::MAX;
    let mut rb = std::i64::MAX;
    for &r in &red {
        let j = green.lower_bound(&r);
        let k = blue.lower_bound(&r);
        if j < green.len() {
            rg = rg.min((r - green[j]).abs());
        }
        if j >= 1 {
            rg = rg.min((green[j - 1] - r).abs());
        }
        if k < blue.len() {
            rb = rb.min((r - blue[k]).abs());
        }
        if k >= 1 {
            rb = rb.min((blue[k - 1] - r).abs());
        }
    }
    let mut ans = rg.saturating_add(rb);
    for &g in &green {
        let j = blue.lower_bound(&g);
        let mut gb = std::i64::MAX;
        if j < blue.len() {
            gb = gb.min((g - blue[j]).abs());
        }
        if j >= 1 {
            gb = gb.min((blue[j - 1] - g).abs());
        }
        ans = ans.min(gb);
    }

    assert!(ans < std::i64::MAX);
    println!("{}", ans);
}
