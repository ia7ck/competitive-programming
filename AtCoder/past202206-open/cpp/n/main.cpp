#include "atcoder/maxflow"
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int h, w;
    cin >> h >> w;
    vector<vector<long long>> c(h, vector<long long>(w));
    REP(i, h) {
        REP(j, w) { cin >> c[i][j]; }
    }

    const long long INF = 1234567891234567890;
    atcoder::mf_graph<long long> mf(h * w * 2);
    vector<int> di = {-1, 0, 0, 1}, dj = {0, -1, 1, 0};
    REP(i, h) {
        REP(j, w) {
            if ((i == 0 && j == 0) || (i == h - 1 && j == w - 1)) {
                mf.add_edge(i * w + j, (h * w) + (i * w + j), INF);
            } else {
                mf.add_edge(i * w + j, (h * w) + (i * w + j), c[i][j]);
            }
            REP(k, 4) {
                auto ni = i + di[k], nj = j + dj[k];
                if (0 <= ni && ni < h && 0 <= nj && nj < w) {
                    mf.add_edge((h * w) + (i * w + j), ni * w + nj, INF);
                }
            }
        }
    }

    cout << mf.flow(0, h * w * 2 - 1) << '\n';
    vector<vector<char>> ans(h, vector<char>(w, '.'));
    auto cut = mf.min_cut(0);
    REP(v, h * w) {
        if (v == 0 || v == h * w - 1) {
            continue;
        }
        if (cut[v] == true && cut[h * w + v] == false) {
            int i = v / w, j = v % w;
            ans[i][j] = '#';
        }
    }
    REP(i, h) {
        REP(j, w) {
            cout << ans[i][j];
        }
        cout << '\n';
    }

    return 0;
}
