#include <algorithm>
#include <iostream>
#include <limits>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

// ford-fulkerson
template <typename T> class Graph {
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

int main() {

  int h, w;
  cin >> h >> w;
  vector<vector<char>> a(h, vector<char>(w));
  int sy, sx, gy, gx;
  rep(i, h) rep(j, w) {
    cin >> a[i][j];
    if (a[i][j] == 'S') {
      sy = i;
      sx = j;
    } else if (a[i][j] == 'T') {
      gy = i;
      gx = j;
    }
  }
  if (sy == gy or sx == gx) {
    cout << -1 << endl;
    return 0;
  }

  auto n = h + w;
  Graph<int> g(n + 2);
  auto S = n, T = n + 1;
  auto inf = numeric_limits<int>::max() / 3;
  g.add_edge(S, sy, inf); // S は壊せない
  g.add_edge(S, sx + h, inf);
  g.add_edge(gy, T, inf); // T は壊せない
  g.add_edge(gx + h, T, inf);
  rep(i, h) rep(j, w) if (a[i][j] == 'o') {
    g.add_edge(i, j + h, 1); // 壊すとコスト1かかる
    g.add_edge(j + h, i, 1);
  }

  cout << g.ford(S, T) << endl;

  return 0;
}
