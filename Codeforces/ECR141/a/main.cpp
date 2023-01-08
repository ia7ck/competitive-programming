#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(const int n, const vector<int> &a) {
    int c = count(a.begin(), a.end(), a[0]);
    if (c == n) {
        cout << "NO\n";
        return;
    }

    cout << "YES\n";
    vector<int> b;
    b.push_back(a[0]);
    b.push_back(a[n - 1]);
    for (int i = 1; i + 1 < n; i++) {
        b.push_back(a[i]);
    }
    int s = 0;
    REP(i, n) {
        if (i >= 1) {
            assert(s != b[i]);
        }
        s += b[i];
    }
    REP(i, n) {
        cout << b[i];
        if (i + 1 < n) {
            cout << ' ';
        } else {
            cout << '\n';
        }
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
        vector<int> a(n);
        REP(i, n) { cin >> a[i]; }
        solve(n, a);
    }

    return 0;
}
