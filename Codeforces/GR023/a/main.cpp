#include <iostream>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(int n, int k, const vector<int> &a) {
    for (auto x: a) {
        if (x == 1) {
            cout << "YES" << endl;
            return;
        }
    }
    cout << "NO" << endl;
}

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int t;
    cin >> t;
    while (t--) {
        int n, k;
        cin >> n >> k;
        vector<int> a(n);
        for (auto &e: a) {
            cin >> e;
        }
        solve(n, k, a);
    }

    return 0;
}
