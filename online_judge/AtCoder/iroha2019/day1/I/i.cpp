#include <algorithm>
#include <deque>
#include <iostream>
#include <map>
#include <queue>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct Edge {
  int to, cost;
};

struct P {
  int v, d;
};

template <typename T> vector<T> dijkstra(vector<vector<Edge>> &g, int s) {
  const auto inf = numeric_limits<T>::max();
  vector<T> dist((int)g.size(), inf);
  dist[s] = 0;
  priority_queue<pair<T, int>> q;
  q.push({-dist[s], s});
  while (q.size() > 0) {
    T cost;
    int u;
    tie(cost, u) = q.top();
    q.pop();
    cost *= -1;
    for (auto e : g[u]) {
      if (cost + e.cost < dist[e.to]) {
        dist[e.to] = cost + e.cost;
        q.push({-dist[e.to], e.to});
      }
    }
  }
  return dist;
}

int main() {

  int n, m, k;
  cin >> n >> m >> k;

  map<pair<int, int>, int> idx;
  vector<vector<Edge>> g(n + m * 2 + 1);
  auto add_edge = [&](pair<int, int> from, pair<int, int> to, int cost) {
    if (!idx.count(from)) idx[from] = idx.size();
    if (!idx.count(to)) idx[to] = idx.size();
    g[idx[from]].push_back(Edge{idx[to], cost});
  };
  rep(i, m) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;
    b--;
    add_edge({a, c}, {b, c}, 0);
    add_edge({b, c}, {a, c}, 0);
    add_edge({a, c}, {a, 0}, 0);
    add_edge({b, c}, {b, 0}, 0);
    add_edge({a, 0}, {b, c}, 1);
    add_edge({b, 0}, {a, c}, 1);
  }

  const auto inf = numeric_limits<int>::max();
  if (!idx.count({0, 0})) idx[{0, 0}] = idx.size();
  auto dist = dijkstra<int>(g, idx[{0, 0}]);
  if (!idx.count({n - 1, 0})) idx[{n - 1, 0}] = idx.size();
  auto ans = (int64_t)dist[idx[{n - 1, 0}]] * k;
  if (ans >= inf) ans = -1;
  cout << ans << endl;

  return 0;
}
