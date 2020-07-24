#include <algorithm>
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

#include <iostream>
#include <map>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
  int n, m, d;
  cin >> n >> m >> d;
  vector<vector<int>> t(n);
  t[0].push_back(0);
  t[n - 1].push_back(2e9);
  vector<tuple<int, int, int, int, int>> a;
  rep(i, m) {
    int u, v, p, q, w;
    cin >> u >> v >> p >> q >> w;
    u--;
    v--;
    t[u].push_back(p);
    t[v].push_back(q + d);
    a.emplace_back(u, v, p, q + d, w);
  }
  map<pair<int, int>, int> mp; // 街, 時刻
  rep(i, n) {
    sort(t[i].begin(), t[i].end());
    t[i].erase(unique(t[i].begin(), t[i].end()), t[i].end());
    for (auto tm : t[i]) {
      mp[{i, tm}] = mp.size();
    }
  }
  Graph<int64_t> g(mp.size());
  auto inf = numeric_limits<int64_t>::max() / 3;
  rep(i, n) {
    for (size_t j = 1; j < t[i].size(); j++) {
      // 飛行機に乗らない
      g.add_edge(mp[{i, t[i][j - 1]}], mp[{i, t[i][j]}], inf);
    }
  }
  rep(i, m) {
    int u, v, p, q, w;
    tie(u, v, p, q, w) = a[i];
    // 飛行機に乗る
    g.add_edge(mp[{u, p}], mp[{v, q}], w);
  }
  auto mf = g.ford(mp[{0, 0}], mp[{n - 1, 2e9}]);
  cout << mf << endl;

  return 0;
}
