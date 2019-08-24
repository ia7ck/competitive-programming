#include <algorithm>
#include <iostream>
#include <limits>
#include <vector>
using namespace std;
// ford-fulkerson
template <typename T>
class Graph {
  struct Edge {
    int to, rev;
    T cap;
    Edge(int to, int rev, T cap) : to(to), rev(rev), cap(cap) {}
  };
  vector<vector<Edge>> g;
  T inf;
  vector<int> visited;

public:
  Graph(int n) {
    g.resize(n);
    inf = numeric_limits<T>::max() / 3;
    visited.resize(n, false);
  }
  void add_edge(int from, int to, T cap) {
    g[from].emplace_back(to, g[to].size(), cap);
    g[to].emplace_back(from, (int)g[from].size() - 1, 0);
  }
  T dfs(int i, T curf, int sink) {
    if (i == sink) return curf;
    visited[i] = true;
    for (auto &&e : g[i]) {
      if (visited[e.to] or e.cap == 0) continue;
      auto tmpf = dfs(e.to, min(e.cap, curf), sink);
      if (tmpf > 0) {
        e.cap -= tmpf;
        g[e.to][e.rev].cap += tmpf;
        return tmpf;
      }
    }
    return 0; // 流せなかった
  }
  T ford(int source, int sink) {
    T maxf = 0;
    while (true) {
      fill(visited.begin(), visited.end(), false);
      auto f = dfs(source, inf, sink);
      if (f > 0) {
        maxf += f;
      } else {
        break;
      }
    }
    return maxf;
  }
};

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n, m;
  cin >> n >> m;
  vector<vector<char>> a(n, vector<char>(m));
  rep(i, n) rep(j, m) cin >> a[i][j];

  Graph<int> g(n * m + 2);
  auto S = n * m, T = S + 1;
  auto index = [&](int i, int j) { return i * m + j; };
  int wh = 0, bk = 0;
  vector<int> dy = {-1, 0, 0, 1}, dx = {0, -1, 1, 0};
  rep(i, n) {
    rep(j, m) {
      if (a[i][j] == 'w') {
        wh += 1;
        g.add_edge(S, index(i, j), 1);
        rep(k, 4) {
          auto ni = i + dy[k], nj = j + dx[k];
          if (0 <= ni and ni < n and 0 <= nj and nj < m) {
            if (a[ni][nj] == 'b') g.add_edge(index(i, j), index(ni, nj), 1);
          }
        }
      } else if (a[i][j] == 'b') {
        bk += 1;
        g.add_edge(index(i, j), T, 1);
      }
    }
  }
  auto match = g.ford(S, T);
  auto ans = match * 100;
  wh -= match;
  bk -= match;
  auto mn = min(wh, bk);
  ans += mn * 10;
  wh -= mn;
  bk -= mn;
  ans += wh + bk;
  cout << ans << endl;

  return 0;
}
