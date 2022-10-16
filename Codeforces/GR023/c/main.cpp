#include <iostream>
#include <algorithm>
#include <tuple>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(int n, const vector<int> &a) {
    vector<tuple<int, int>> di;
    REP(i, n - 1) {
        di.emplace_back(a[i] - a[i + 1], i);
    }
    sort(di.begin(), di.end());
    vector<int> ans;
    for (auto [d, i]: di) {
        while (d > 0) {
            int j = ans.size() + 1;
            d -= j;
            ans.push_back(i + 2);
        }
    }
    while (ans.size() < n) {
        ans.push_back(1);
    }
    REP(i, n) {
        cout << ans[i];
        if (i + 1 < n) {
            cout << " ";
        }
    }
    cout << "\n";
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
