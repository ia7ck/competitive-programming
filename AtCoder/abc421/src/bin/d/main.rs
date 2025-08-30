use proconio::input;

fn main() {
    input! {
        r_t: i64,
        c_t: i64,
        r_a: i64,
        c_a: i64,
        _n: usize,
        m: usize,
        l: usize,
        sa: [(char, usize); m],
        tb: [(char, usize); l],
    };

    enum State {
        Tak { s: char, a: usize },
        Aok { t: char, b: usize },
    }

    let mut r = r_a - r_t;
    let mut c = c_a - c_t;
    let mut state = Option::<State>::None;
    let mut ans = 0_usize;
    let mut i = 0;
    let mut j = 0;
    while i < sa.len() || j < tb.len() {
        state = match state {
            None => {
                let (s, a) = sa[i];
                let (t, b) = tb[j];
                i += 1;
                j += 1;
                let (dr, dc) = dir(t);
                let (dr_s, dc_s) = dir(s);
                let (dr, dc) = (dr - dr_s, dc - dc_s);
                if a > b {
                    if dr != 0 {
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= b as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= b as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += b;
                        }
                    }
                    (r, c) = (r + dr * b as i64, c + dc * b as i64);
                    Some(State::Tak { s, a: a - b })
                } else if a < b {
                    if dr != 0 {
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= a as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= a as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += a;
                        }
                    }
                    (r, c) = (r + dr * a as i64, c + dc * a as i64);
                    Some(State::Aok { t, b: b - a })
                } else {
                    // (r, c) -> (r + dr * a, c + dc * a)
                    if dr != 0 {
                        // r + dr * k = 0
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= a as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        // c + dc * k = 0
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= a as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += a;
                        }
                    }
                    (r, c) = (r + dr * a as i64, c + dc * a as i64);
                    None
                }
            }
            Some(State::Tak { s, a }) => {
                let (t, b) = tb[j];
                j += 1;
                let (dr, dc) = dir(t);
                let (dr_s, dc_s) = dir(s);
                let (dr, dc) = (dr - dr_s, dc - dc_s);
                if a > b {
                    if dr != 0 {
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= b as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= b as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += b;
                        }
                    }
                    (r, c) = (r + dr * b as i64, c + dc * b as i64);
                    Some(State::Tak { s, a: a - b })
                } else if a < b {
                    if dr != 0 {
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= a as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= a as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += a;
                        }
                    }
                    (r, c) = (r + dr * a as i64, c + dc * a as i64);
                    Some(State::Aok { t, b: b - a })
                } else {
                    // (r, c) -> (r + dr * a, c + dc * a)
                    if dr != 0 {
                        // r + dr * k = 0
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= a as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        // c + dc * k = 0
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= a as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += a;
                        }
                    }
                    (r, c) = (r + dr * a as i64, c + dc * a as i64);
                    None
                }
            }
            Some(State::Aok { t, b }) => {
                let (s, a) = sa[i];
                i += 1;
                let (dr, dc) = dir(t);
                let (dr_s, dc_s) = dir(s);
                let (dr, dc) = (dr - dr_s, dc - dc_s);
                if a > b {
                    if dr != 0 {
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= b as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= b as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += b;
                        }
                    }
                    (r, c) = (r + dr * b as i64, c + dc * b as i64);
                    Some(State::Tak { s, a: a - b })
                } else if a < b {
                    if dr != 0 {
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= a as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= a as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += a;
                        }
                    }
                    (r, c) = (r + dr * a as i64, c + dc * a as i64);
                    Some(State::Aok { t, b: b - a })
                } else {
                    // (r, c) -> (r + dr * a, c + dc * a)
                    if dr != 0 {
                        // r + dr * k = 0
                        if r % dr == 0 {
                            let k = -r / dr;
                            if 1 <= k && k <= a as i64 && c + dc * k == 0 {
                                ans += 1;
                            }
                        }
                    } else if dc != 0 {
                        // c + dc * k = 0
                        if c % dc == 0 {
                            let k = -c / dc;
                            if 1 <= k && k <= a as i64 && r + dr * k == 0 {
                                ans += 1;
                            }
                        }
                    } else {
                        if (r, c) == (0, 0) {
                            ans += a;
                        }
                    }
                    (r, c) = (r + dr * a as i64, c + dc * a as i64);
                    None
                }
            }
        };
        // dbg!(i, j, r, c, ans);
    }

    println!("{}", ans);
}

fn dir(d: char) -> (i64, i64) {
    match d {
        'U' => (-1, 0),
        'D' => (1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => unreachable!(),
    }
}
