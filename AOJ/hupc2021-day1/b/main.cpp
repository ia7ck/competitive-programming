#include <iostream>
#include <vector>

using namespace std;

#define REP(i, n) for (int (i) = 0; (i) < (n); (i)++)

int main() {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto &e: a) {
        cin >> e;
    }

    vector<int64_t> in(n, 0);
    REP(i, n) {
        in[i + a[i] - 1] += 1;
    }
    int64_t mod = 998244353;
    int64_t ans = 1;
    int64_t cur = 0;
    for (int i = n - 1; i >= 0; i--) {
        (cur += in[i]) %= mod;
        (ans *= (cur - (n - 1 - i))) %= mod;
    }
    cout << ans << endl;
}
