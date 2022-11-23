#include <iostream>
#include <vector>
#include <algorithm>
#include "atcoder/convolution"

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);
    
    int n;
    cin >> n;
    vector<int> a(n);
    REP(i, n) {
        cin >> a[i];
    }

    int max_a = *max_element(a.begin(), a.end());
    vector<long long> p(max_a + 1), q(max_a + 1);
    for (const auto &x: a) {
        p[x] += 1;
        q[max_a - x] += 1;
    }

    auto r = atcoder::convolution_ll(p, q);
    // r[a[i] - a[j] + max_a] ... a[i] - a[j] が書かれる回数
    // a[i] - a[j] + max_a >= 0

    int ans = 0;
    for (const auto &c: r) {
        if (c >= 1) {
            ans += 1;
        }
    }
    cout << ans << '\n';

    
    return 0;
}
