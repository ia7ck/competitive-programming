#include "atcoder/mincostflow"
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int n, k;
    cin >> n >> k;
    vector<vector<long long>> a(n, vector<long long>(n));
    REP(i, n) {
        REP(j, n) { cin >> a[i][j]; }
    }

    const int M = 3000;
    const long long X = 1'000'000'000'000'000;
    atcoder::mcf_graph<int, long long> mcf_g(n * 2 + 2);
    int source = n * 2, sink = n * 2 + 1;
    REP(i, n) { mcf_g.add_edge(source, i, k, 0); }
    REP(j, n) { mcf_g.add_edge(n + j, sink, k, 0); }
    REP(i, n) {
        REP(j, n) { mcf_g.add_edge(i, n + j, 1, X - a[i][j]); }
    }
    mcf_g.add_edge(source, sink, M, X);

    auto [flow, cost] = mcf_g.flow(source, sink, M);
    cout << X * M - cost << '\n';

    vector<vector<char>> ans(n, vector<char>(n, '.'));
    for (const auto &e : mcf_g.edges()) {
        if (e.from == source || e.to == sink) {
            continue;
        }
        if (e.cap == e.flow) {
            int i = e.from, j = e.to - n;
            ans[i][j] = 'X';
        }
    }

    REP(i, n) {
        REP(j, n) { cout << ans[i][j]; }
        cout << '\n';
    }
    return 0;
}
