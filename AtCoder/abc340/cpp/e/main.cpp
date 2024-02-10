#include <iostream>
#include <cstdio>
#include <vector>

#include "atcoder/lazysegtree"

#define REP(i, n) for (int i = 0; i < (int)(n); i++)

using ll = long long;

ll op(ll a, ll b) {
    // not matter
    return std::max(a, b);
}

ll e() {
    return 0;
}

ll mapping(ll f, ll x) {
    return f + x;
}

ll composition(ll f, ll g) {
    return f + g;
}

ll id() {
    return 0;
}

int main() {
    int n, m;
    scanf("%d%d", &n, &m);
    std::vector<ll> a(n);
    REP(i, n) {
        scanf("%lld", &a[i]);
    }
    std::vector<int> b(m);
    REP(i, m) {
        scanf("%d", &b[i]);
    }

    atcoder::lazy_segtree<ll, op, e, ll, mapping, composition, id> seg(a);
    for (const auto j: b) {
        auto x = seg.get(j);
        seg.set(j, 0);
        auto q = x / (ll)n;
        auto r = x % (ll)n;
        seg.apply(0, n, q);
        if (r >= 1) {
            auto s = (j + 1) % n;
            auto t = (j + r + 1) % n;
            if (s <= t) {
                seg.apply(s, t, 1);
            } else {
                seg.apply(s, n, 1);
                seg.apply(0, t, 1);
            }
        }
    }

    REP(i, n) {
        printf("%lld", seg.get(i));
        if (i + 1 < n) {
            printf(" ");
        } else {
            printf("\n");
        }
    }

    return 0;
}
