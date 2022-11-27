#include "atcoder/lazysegtree"
#include <algorithm>
#include <iostream>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

struct S {
    // black_dy = 0, white_dy > 0
    // black_dy > 0, white_dy = 0
    long long black_dy, white_dy;
};

struct F {
    bool flip;
};

S op(S l, S r) { return S{l.black_dy + r.black_dy, l.white_dy + r.white_dy}; }

S e() { return S{0, 0}; };

S mapping(F f, S s) {
    if (f.flip) {
        return S{s.white_dy, s.black_dy};
    } else {
        return s;
    }
}

F composition(F f, F g) {
    // xor
    return F{f.flip != g.flip};
}

F id() { return F{false}; }

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int q;
    cin >> q;
    vector<tuple<long long, long long, long long>> xyy;
    vector<long long> y_values;
    REP(i, q) {
        long long a, b, c, d;
        cin >> a >> b >> c >> d;
        xyy.emplace_back(a, b, d);
        xyy.emplace_back(c, b, d);
        y_values.push_back(b);
        y_values.push_back(d);
    }

    sort(y_values.begin(), y_values.end());
    y_values.erase(unique(y_values.begin(), y_values.end()), y_values.end());

    // seg[0] : y_values[0] 〜 y_values[1]
    // seg[1] : y_values[1] 〜 y_values[2]
    // ...
    atcoder::lazy_segtree<S, op, e, F, mapping, composition, id> seg(y_values.size() - 1);
    REP(i, y_values.size() - 1) { seg.set(i, S{0, y_values[i + 1] - y_values[i]}); }

    long long ans = 0;
    sort(xyy.begin(), xyy.end());
    REP(i, xyy.size()) {
        auto [x, y_start, y_end] = xyy[i];
        if (i >= 1) {
            auto prev_x = get<0>(xyy[i - 1]);
            ans += (x - prev_x) * seg.all_prod().black_dy;
        }
        y_start = lower_bound(y_values.begin(), y_values.end(), y_start) - y_values.begin();
        y_end = lower_bound(y_values.begin(), y_values.end(), y_end) - y_values.begin();
        seg.apply(y_start, y_end, F{true});
    }

    cout << ans << '\n';

    return 0;
}
