use ac_library::maxflow;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[u64; w]; h],
    };

    let mut ans = {
        let mut x = 0;
        for i in 0..h {
            for j in 0..w {
                x ^= a[i][j];
            }
        }
        x
    };
    for bits in 0..(1_usize << (h * w)) {
        let mut board = vec![vec![false; w]; h];
        for i in 0..h {
            for j in 0..w {
                board[i][j] = bits >> (i * w + j) & 1 == 1;
            }
        }

        let mut g = maxflow::MfGraph::<u32>::new(h * w + 2);
        let source = h * w;
        let sink = h * w + 1;
        for i in 0..h {
            for j in 0..w {
                if board[i][j] == false {
                    continue;
                }
                if (i + j) % 2 == 0 {
                    g.add_edge(source, i * w + j, 1);
                    if i + 1 < h && board[i + 1][j] {
                        g.add_edge(i * w + j, (i + 1) * w + j, 1);
                    }
                    if i >= 1 && board[i - 1][j] {
                        g.add_edge(i * w + j, (i - 1) * w + j, 1);
                    }
                    if j + 1 < w && board[i][j + 1] {
                        g.add_edge(i * w + j, i * w + j + 1, 1);
                    }
                    if j >= 1 && board[i][j - 1] {
                        g.add_edge(i * w + j, i * w + j - 1, 1);
                    }
                } else {
                    g.add_edge(i * w + j, sink, 1);
                }
            }
        }
        if g.flow(source, sink) * 2 == bits.count_ones() {
            let new_ans = {
                let mut x = 0;
                for i in 0..h {
                    for j in 0..w {
                        if board[i][j] == false {
                            x ^= a[i][j];
                        }
                    }
                }
                x
            };
            ans = ans.max(new_ans);
        }
    }

    println!("{}", ans);
}
