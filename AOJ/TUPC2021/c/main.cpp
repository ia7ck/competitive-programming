#include<iostream>
#include<vector>
#include<algorithm>
#include<cassert>

using namespace std;
#define REP(i, n) for (int (i) = 0; (i) < (int)(n); (i)++)

int memo[3][2001][2001]; // 0: lose, 1: win

bool judge(int turn, int one, int two) {
    if (one == 0 && two == 0) {
        return false;
    }
    auto cache = memo[turn % 3][one][two];
    if (cache <= 1) {
        return cache;
    }

    vector<bool> next;
    int add = (turn + 1) % 3 == 0;
    if (one >= 1) {
        next.push_back(judge(turn + 1, one - 1, two + add));
    }
    if (two >= 1) {
        next.push_back(judge(turn + 1, one + 1, two - 1 + add));
        next.push_back(judge(turn + 1, one, two - 1 + add));
    }
    assert(!next.empty());
    auto current_player_win = any_of(next.begin(), next.end(), [](bool next_player_win) {
        return !next_player_win;
    });
    return memo[turn % 3][one][two] = current_player_win;
}

int main() {
    int n;
    cin >> n;

    REP(t, 3) {
        REP(i, 2001) {
            REP(j, 2001) {
                memo[t][i][j] = 2;
            }
        }
    }

    auto ans = judge(0, 0, n);
    if (ans) {
        cout << "Alice" << endl;
    } else {
        cout << "Bob" << endl;
    }

    return 0;
}

// 2 2

// A: 0 2
// B: 0 0

// A: 1 2
// B: 1 0
// A: 0 0 2
// B: 0 0 0


// 2 2 2

// A: 0 2 2
// B: ...
