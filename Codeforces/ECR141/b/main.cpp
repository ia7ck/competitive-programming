#include <cassert>
#include <iostream>
#include <set>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(const int n) {
    vector<vector<int>> a(n, vector<int>(n));
    vector<int> values;
    for (int x = 1; x <= n * n; x++) {
        if (x % 2 == 1) {
            values.push_back(x / 2 + 1);
        } else {
            values.push_back(n * n - (x - 2) / 2);
        }
    }
    vector<int> di = {0, 1, 0, -1};
    vector<int> dj = {1, 0, -1, 0};
    int p = 0, i = 0, j = 0;
    for (const auto x : values) {
        assert(a[i][j] == 0);
        a[i][j] = x;
        if (0 <= i + di[p] && i + di[p] < n && 0 <= j + dj[p] && j + dj[p] < n &&
            a[i + di[p]][j + dj[p]] == 0) {
            i += di[p];
            j += dj[p];
        } else {
            p += 1;
            p %= 4;
            i += di[p];
            j += dj[p];
        }
        assert(0 <= i && i < n && 0 <= j && j < n);
    }
    set<int> diff;
    REP(i, n) {
        REP(j, n) {
            if (i + 1 < n) {
                diff.insert(abs(a[i][j] - a[i + 1][j]));
            }
            if (j + 1 < n) {
                diff.insert(abs(a[i][j] - a[i][j + 1]));
            }
        }
    }
    assert(diff.size() == n * n - 1);
    REP(i, n) {
        REP(j, n) {
            cout << a[i][j];
            if (j + 1 < n) {
                cout << ' ';
            }
        }
        cout << '\n';
    }
}

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int t;
    cin >> t;
    while (t--) {
        int n;
        cin >> n;
        solve(n);
    }

    return 0;
}
