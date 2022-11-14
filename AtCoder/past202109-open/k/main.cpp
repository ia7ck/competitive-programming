#include <iostream>
#include <numeric>
#include <vector>
#include "atcoder/mincostflow"

#define REP(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

using ll = long long;

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int p, q;
    cin >> p >> q;
    vector<string> s(p);
    REP(i, p) {
        cin >> s[i];
    }

    vector<ll> a(p), b(p);
    REP(i, p) {
        cin >> a[i] >> b[i];
    }
    vector<ll> c(q), d(q);
    REP(j, q) {
        cin >> c[j] >> d[j];
    }

    auto base = accumulate(b.begin(), b.end(), 0LL) + accumulate(d.begin(), d.end(), 0LL);

    ll INF = 1e15;
    atcoder::mcf_graph<int, ll> g(p + q + 2);
    auto source = p + q, sink = p + q + 1;
    REP(i, p) {
        g.add_edge(source, i, 1, 0);
    }
    REP(j, q) {
        g.add_edge(p + j, sink, 1, 0);
    }
    REP(i, p) {
        REP(j, q) {
            if (s[i][j] == '0') {
                continue;
            }
            auto y = a[i] + c[j] - b[i] - d[j];
            if (y > 0) {
                g.add_edge(i, p + j, 1, INF - y);
            }
        }
    }

    ll ans = 0;
    for (auto [flow, cost]: g.slope(source, sink)) {
        // i -> j の辺を flow 回使った
        ans = max(ans, base + (INF * flow - cost));
    }

    cout << ans << '\n';

    return 0;
}
