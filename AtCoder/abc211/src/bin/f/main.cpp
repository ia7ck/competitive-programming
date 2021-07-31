#include<iostream>
#include<vector>
#include<algorithm>
#include<tuple>
#include <atcoder/lazysegtree>

using namespace std;

#define rep(i, n) for (int (i) = 0; (i) < (int)(n); (i)++)

using S = long long;
using F = long long;

S op(S l, S r) {
    return l;
}

S e() {
    return 0;
}

S mapping(F l, S r) {
    return l + r;
}

F composition(F l, F r) {
    return l + r;
}

F id() {
    return 0;
}

int main() {

    int n;
    cin >> n;
    const int Z = 100000 + 1;
    vector<vector<tuple<int, int, int>>> vert_lines(Z);
    atcoder::lazy_segtree<S, op, e, F, mapping, composition, id> seg(Z);
    rep(_, n) {
        int m;
        cin >> m;
        vector<int> x(m), y(m);
        rep(i, m) {
            cin >> x[i] >> y[i];
        }
        vector<tuple<int, int, int>> v_lines;
        rep(i, m) {
            if (i % 2 == 0) {
                assert(x[i] == x[(i + 1) % m]);
                auto y1 = min(y[i], y[(i + 1) % m]);
                auto y2 = max(y[i], y[(i + 1) % m]);
                v_lines.emplace_back(x[i], y1, y2);
            }
        }
        sort(v_lines.begin(), v_lines.end());
        for (auto[x, y1, y2]: v_lines) {
            auto incl = seg.get(y1);
            if (incl == 0) {
                seg.apply(y1, y2, 1);
                vert_lines[x].emplace_back(1, y1, y2);
            } else {
                seg.apply(y1, y2, -1);
                vert_lines[x].emplace_back(-1, y1, y2);
            }
        }
    }
    int q;
    cin >> q;
    vector<vector<pair<int, int>>> yi(Z);
    rep(i, q) {
        int x, y;
        cin >> x >> y;
        yi[x].emplace_back(y, i);
    }

    vector<S> ans(q);
    rep(x, Z) {
        for (auto[sgn, y1, y2]: vert_lines[x]) {
            if (sgn == 1) {
                seg.apply(y1, y2, 1);
            } else {
                seg.apply(y1, y2, -1);
            }
        }
        for (auto[y, i] : yi[x]) {
            ans[i] = seg.get(y);
        }
    }

    for (auto a: ans) {
        cout << a << endl;
    }


    return 0;
}
