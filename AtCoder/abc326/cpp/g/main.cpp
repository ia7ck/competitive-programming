#include <cstdio>
#include <vector>
#include <numeric>
#include "atcoder/maxflow"

#define REP(i, n) for (int i = 0; i < (n); i++)

int main() {
    int n, m;
    scanf("%d%d", &n, &m);
    std::vector<int> cost(n);
    REP(i, n) {
        scanf("%d", &cost[i]);
    }
    std::vector<int> achieve(m);
    REP(i, m) {
        scanf("%d", &achieve[i]);
    }
    std::vector<std::vector<int>> level(m, std::vector<int>(n, 0));
    REP(i, m) {
        REP(j, n) {
            scanf("%d", &level[i][j]);
        }
    }

    auto id = [&](int skill, int level) {
        return m + skill * 5 + level;
    };

    const int INF = 1'000'000'000;
    atcoder::mf_graph<int> g(m + n * 5 + 2);
    int s = m + n * 5;
    int t = s + 1;
    REP(i, m) {
        g.add_edge(s, i, achieve[i]);
        REP(j, n) {
            g.add_edge(i, id(j, level[i][j] - 1), INF);
        }
    }
    REP(j, n) {
        REP(k, 4) {
            g.add_edge(id(j, k + 1), id(j, k), INF);
        }
        for (int k = 1; k < 5; k++) {
            g.add_edge(id(j, k), t, cost[j]);
        }
        g.add_edge(id(j, 0), t, 0);
    }

    int ans = std::accumulate(achieve.begin(), achieve.end(), 0) - g.flow(s, t);
    printf("%d\n", ans);

    return 0;
}
