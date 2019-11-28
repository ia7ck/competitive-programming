#include <algorithm>
#include <limits>
#include <vector>

using namespace std;

// ford-fulkerson
template <typename T>
struct MaxFlow {
  struct Edge {
    int to, rev;
    T cap;
    Edge(int to, int rev, T cap) : to(to), rev(rev), cap(cap) {}
  };
  vector<vector<Edge>> g;
  T inf;
  vector<int> visited;

  MaxFlow(int n) {
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

#include <iostream>

int main() {

  int n, k, m;
  cin >> n >> k >> m;
  auto g = MaxFlow<int>(n + 1);
  for (int i = 0; i < k; i++) {
    int p;
    cin >> p;
    g.add_edge(n, p, 1);
  }
  for (int i = 0; i < m; i++) {
    int a, b;
    cin >> a >> b;
    g.add_edge(a, b, 1);
    g.add_edge(b, a, 1);
  }
  auto mxf = g.ford(n, 0);
  cout << mxf << endl;

  return 0;
}

// https://atcoder.jp/contests/abc010/submissions/8686040
