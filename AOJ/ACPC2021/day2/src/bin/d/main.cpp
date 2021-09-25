#include<iostream>
#include<vector>
#include<algorithm>
#include "atcoder/segtree"

using namespace std;
#define rep(i, n) for (int (i) = 0; (i) < (n); (i)++)

int64_t op(int64_t a, int64_t b) {
    return max(a, b);
}

int64_t e() {
    return 0;
}

int main() {

    int n;
    cin >> n;
    vector<int> a(n);
    for (auto &e: a) {
        cin >> e;
    }
    vector<int64_t> w(n);
    for (auto &e: w) {
        cin >> e;
    }

    vector<int> u = a;
    sort(u.begin(), u.end());
    u.erase(unique(u.begin(), u.end()), u.end());
    atcoder::segtree <int64_t, op, e> seg((int) u.size());
    rep(i, n) {
        auto x = lower_bound(u.begin(), u.end(), a[i]) - u.begin();
        auto s = seg.prod(0, x) + w[i];
        seg.set(x, max(seg.get(x), s));
    }
    cout << seg.all_prod() << endl;

    return 0;
}
