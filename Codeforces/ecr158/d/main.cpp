#include <vector>
#include <cstdio>
#include "atcoder/segtree"
#include <iostream>

#define REP(i, n) for (int i = 0; i < (int)(n); i++)

using namespace std;
using ll = long long;

using S = ll;

S op(S l, S r) { return max(l, r); }

S e() { return -1e18; }

int main() {
    int n;
    scanf("%d", &n);
    vector<ll> a(n);
    REP(i, n) {
        scanf("%lld", &a[i]);
    }

    vector<ll> b = a;
    for (int i = 1; i < n; i++) {
        b[i] += i;
    }
    atcoder::segtree<S, op, e> seg(b);
    ll ans = 1e18;
    REP(i, n) {
        auto max_ = a[i];
        max_ = max(max_, seg.prod(0, i));
        max_ = max(max_, seg.prod(i + 1, n));
        ans = min(ans, max_);
        seg.set(i, a[i] + (n - i - 1));
    }
    printf("%lld\n", ans);
    return 0;
}
