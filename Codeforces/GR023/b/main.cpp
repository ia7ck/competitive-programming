#include <iostream>
#include <cassert>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(int n, const vector<int> &a) {
    int ans = 0;
    int j = n - 1;
    REP(i, n) {
        if (a[i] == 0) {
            continue;
        }
        while (i < j && a[j] == 1) {
            j -= 1;
        }
        if (i < j) {
            assert(a[j] == 0);
            ans += 1;
            j -= 1;
        }
    }
    cout << ans << "\n";
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
        for (auto &e: a) {
            cin >> e;
        }
        solve(n, a);
    }

    return 0;
}
