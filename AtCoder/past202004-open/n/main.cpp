#include "atcoder/lazysegtree"
#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

struct S {
    long long a;
};

struct F {
    long long b;
};

S op(S l, S r) { return S{l.a + r.a}; }

S e() { return S{0}; }

S mapping(F f, S s) { return S{f.b + s.a}; }

F composition(F f, F g) { return F{f.b + g.b}; }

F id() { return F{0}; }

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int n, q;
    cin >> n >> q;
    map<int, vector<tuple<int, int, long long>>> in;
    map<int, vector<tuple<int, int, long long>>> out;
    vector<int> x_values, y_values;
    REP(i, n) {
        int x_min, y_min, d;
        long long c;
        cin >> x_min >> y_min >> d >> c;
        in[x_min].emplace_back(y_min, y_min + d, c);
        out[x_min + d].emplace_back(y_min, y_min + d, c);
        x_values.push_back(x_min);
        x_values.push_back(x_min + d);
        y_values.push_back(y_min);
        y_values.push_back(y_min + d);
    }
    map<int, vector<pair<int, int>>> queries; // index, y
    REP(i, q) {
        int a, b;
        cin >> a >> b;
        queries[a].emplace_back(i, b);
        x_values.push_back(a);
        y_values.push_back(b);
    }

    sort(x_values.begin(), x_values.end());
    x_values.erase(unique(x_values.begin(), x_values.end()), x_values.end());

    sort(y_values.begin(), y_values.end());
    y_values.erase(unique(y_values.begin(), y_values.end()), y_values.end());

    vector<long long> ans(q);
    atcoder::lazy_segtree<S, op, e, F, mapping, composition, id> seg(y_values.size() + 1);
    for (const auto &x : x_values) {
        for (auto &&[y, yy, c] : in[x]) {
            y = lower_bound(y_values.begin(), y_values.end(), y) - y_values.begin();
            yy = lower_bound(y_values.begin(), y_values.end(), yy) - y_values.begin();
            seg.apply(y, yy + 1, F{c});
        }
        for (auto &&[index, y] : queries[x]) {
            y = lower_bound(y_values.begin(), y_values.end(), y) - y_values.begin();
            ans[index] = seg.get(y).a;
        }
        for (auto &&[y, yy, c] : out[x]) {
            y = lower_bound(y_values.begin(), y_values.end(), y) - y_values.begin();
            yy = lower_bound(y_values.begin(), y_values.end(), yy) - y_values.begin();
            seg.apply(y, yy + 1, F{-c});
        }
    }

    REP(i, q) {
        cout << ans[i] << '\n';
    }

    return 0;
}
