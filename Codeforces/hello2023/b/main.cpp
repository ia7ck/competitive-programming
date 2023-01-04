#include <cassert>
#include <iostream>
#include <numeric>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(int n) {
    if (n == 2) {
        cout << "YES" << '\n';
        cout << "9 5" << '\n';
        return;
    }
    if (n == 3) {
        cout << "NO" << '\n';
        return;
    }

    cout << "YES" << '\n';
    vector<int> a;

    if (n % 2 == 0) {
        REP(i, n) {
            if (i % 2 == 0) {
                a.push_back(-1);
            } else {
                a.push_back(1);
            }
        }
    } else {
        int low = (n - 3) / 2;
        int high = low + 1;
        // high * ((n - 1) / 2) - low * ((n - 1) / 2 + 1)
        // = (n - 1) / 2 - low = 1
        REP(i, n) {
            if (i % 2 == 0) {
                a.push_back(-low);
            } else {
                a.push_back(high);
            }
        }
    }

    assert(a.size() == n);
    int total = accumulate(a.begin(), a.end(), 0);
    REP(i, n) {
        assert(a[i] != 0);
        if (i + 1 < n) {
            assert(a[i] + a[i + 1] == total);
        }
    }
    REP(i, n) {
        cout << a[i];
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
        solve(n);
    }

    return 0;
}
