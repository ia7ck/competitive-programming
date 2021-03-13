#include<iostream>
#include<tuple>
#include<atcoder/maxflow>

using namespace atcoder;
using namespace std;

#define rep(i, n) for(int i = 0; i < (int)(n); i++)

int main() {

    int n, m;
    cin >> n >> m;
    vector<string> s(n);
    rep(i, n) cin >> s[i];

    mf_graph<int> g(n * m + 2);
    auto S = n * m;
    auto T = n * m + 1;

    auto index = [&](int i, int j) {
        return i * m + j;
    };

    // 偶数→奇数
    vector<int> dy = {-1, 0, 0, 1};
    vector<int> dx = {0, -1, 1, 0};
    rep(i, n) {
        rep(j, m) {
            if (s[i][j] == '#') { continue; }
            if ((i + j) % 2 == 0) {
                g.add_edge(S, index(i, j), 1);
            } else {
                g.add_edge(index(i, j), T, 1);
            }
            if ((i + j) % 2 == 1) { continue; }
            rep(k, 4) {
                auto ni = i + dy[k];
                auto nj = j + dx[k];
                if (0 <= ni && ni < n && 0 <= nj && nj < m) {
                    if (s[ni][nj] == '#') { continue; }
                    g.add_edge(index(i, j), index(ni, nj), 1);
                }
            }
        }
    }
    auto ans = g.flow(S, T);
    cout << ans << endl;
    auto rev = [&](int idx) {
        return make_pair(idx / m, idx % m);
    };
    for (auto &e: g.edges()) {
        if (e.from == S || e.to == T) { continue; }
        if (e.flow == 0) { continue; }
        int i, j;
        tie(i, j) = rev(e.from);
        int ni, nj;
        tie(ni, nj) = rev(e.to);
        if (i + 1 == ni) {
            s[i][j] = 'v';
            s[ni][nj] = '^';
        } else if (ni + 1 == i) {
            s[ni][nj] = 'v';
            s[i][j] = '^';
        } else if (j + 1 == nj) {
            s[i][j] = '>';
            s[ni][nj] = '<';
        } else {
            s[ni][nj] = '>';
            s[i][j] = '<';
        }
    }
    rep(i, n) cout << s[i] << endl;

    return 0;
}
