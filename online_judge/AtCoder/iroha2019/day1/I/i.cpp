#include <algorithm>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct Edge {
  int to, c;
};

struct P {
  int d, v, pc;
  bool operator<(const auto &rhs) const { return d > rhs.d; }
};

int main() {

  int n, m, k;
  cin >> n >> m >> k;
  vector<vector<Edge>> g(n);
  map<pair<int, int>, int> edge_map;
  rep(i, m) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;
    b--;
    g[a].push_back(Edge{b, c});
    g[b].push_back(Edge{a, c});
    edge_map[{min(a, b), max(a, b)}] = c;
  }
  const int inf = 1000000000;
  vector<map<int, int>> dist(n);
  rep(i, n) {
    for (auto e : g[i]) {
      dist[i][e.c] = inf;
    }
  }
  priority_queue<P> q;
  for (auto e : g[0]) {
    dist[e.to][e.c] = 0;
    q.push(P{0, e.to, e.c});
  }
  while (q.size() > 0) {
    auto cur = q.top();
    q.pop();
    for (auto e : g[cur.v]) {
      auto nd = cur.d + (e.c == cur.pc ? 0 : 1);
      if (nd < dist[e.to][e.c]) {
        dist[e.to][e.c] = nd;
        q.push(P{nd, e.to, e.c});
      }
    }
  }
  auto mn = inf;
  for (auto pair : dist[n - 1]) {
    mn = min(mn, pair.second);
  }
  if (mn == inf) {
    cout << -1 << endl;
  } else {
    cout << (int64_t)(mn + 1) * k << endl;
  }
  return 0;
}
