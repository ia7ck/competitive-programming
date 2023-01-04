#include <algorithm>
#include <cassert>
#include <iostream>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(int n, string s) {
    int l = count(s.begin(), s.end(), 'L');
    int r = count(s.begin(), s.end(), 'R');
    if (l == n or r == n) {
        cout << -1 << '\n';
        return;
    }

    REP(i, n - 1) {
        if (s[i] == 'L' and s[i + 1] == 'R') {
            cout << i + 1 << '\n';
            return;
        }
    }

    cout << 0 << '\n';
}

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int t;
    cin >> t;
    while (t--) {
        int n;
        cin >> n;
        string s;
        cin >> s;
        solve(n, s);
    }

    return 0;
}
