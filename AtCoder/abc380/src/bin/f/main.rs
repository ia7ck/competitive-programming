use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [u32; n],
        b: [u32; m],
        c: [u32; l],
    };

    let mut loc = [Loc::Table; 12];
    let mut cards = Vec::new();
    for i in 0..n {
        loc[i] = Loc::Takahashi;
        cards.push(a[i]);
    }
    for i in 0..m {
        loc[n + i] = Loc::Aoki;
        cards.push(b[i]);
    }
    for i in 0..l {
        loc[n + m + i] = Loc::Table;
        cards.push(c[i]);
    }

    let ans = solve(Player::Takahashi, loc, &cards, &mut HashMap::new());
    if ans {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

fn solve(player: Player, loc: [Loc; 12], cards: &Vec<u32>, memo: &mut HashMap<(Player, [Loc; 12]), bool>) -> bool {
    if let Some(&win) = memo.get(&(player, loc)) {
        return win;
    }
    let mut f = || match player {
        Player::Takahashi => {
            let new_player = Player::Aoki;
            let mut new_loc = loc;
            for i in 0..cards.len() {
                if matches!(new_loc[i], Loc::Takahashi) {
                    new_loc[i] = Loc::Table;
                    if !solve(new_player, new_loc, cards, memo) {
                        return true;
                    }
                    for j in 0..cards.len() {
                        if i != j && matches!(new_loc[j], Loc::Table) && cards[j] < cards[i] {
                            new_loc[j] = Loc::Takahashi;
                            if !solve(new_player, new_loc, cards, memo) {
                                return true;
                            }
                            new_loc[j] = Loc::Table;
                        }
                    }
                    new_loc[i] = Loc::Takahashi;
                }
            }
            false
        }
        Player::Aoki => {
            let new_player = Player::Takahashi;
            let mut new_loc = loc;
            for i in 0..cards.len() {
                if matches!(new_loc[i], Loc::Aoki) {
                    new_loc[i] = Loc::Table;
                    if !solve(new_player, new_loc, cards, memo) {
                        return true;
                    }
                    for j in 0..cards.len() {
                        if i != j && matches!(new_loc[j], Loc::Table) && cards[j] < cards[i] {
                            new_loc[j] = Loc::Aoki;
                            if !solve(new_player, new_loc, cards, memo) {
                                return true;
                            }
                            new_loc[j] = Loc::Table;
                        }
                    }
                    new_loc[i] = Loc::Aoki;
                }
            }
            false
        }
    };
    let win = f();
    memo.insert((player, loc), win);
    win
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Player {
    Takahashi,
    Aoki,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Loc {
    Takahashi,
    Aoki,
    Table,
}
