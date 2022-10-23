#include <iostream>
#include <vector>
#include <cassert>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

// n = 9
// 5, 1, 6, 2, 7, 3, 8, 4, 9
// n = 10
// 6, 1, 7, 2, 8, 3, 9, 4, 10, 5
void solve(int n) {
    vector<int> a, b;
    for (int i = 1; i <= n / 2; i++) {
        a.push_back(i);
    }
    for (int i = n / 2 + 1; i <= n; i++) {
        b.push_back(i);
    }
    vector<int> ans(n);
    REP(i, n / 2) {
        ans[i * 2 + 1] = a[i];
    }
    REP(i, n - n / 2) {
        ans[i * 2] = b[i];
    }
    REP(i, n) {
        assert(ans[i] != 0);
        cout << ans[i];
        if (i + 1 < n) {
            cout << ' ';
        }
    }
    cout << '\n';
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
