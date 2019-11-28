#include <cassert>
#include <limits>
#include <queue>
#include <tuple>
#include <vector>

using namespace std;

template <typename T> struct Edge {
  int to;
  T cost;
};

template <typename T> vector<T> dijkstra(vector<vector<Edge<T>>> &g, int s) {
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
  const int n = 4, m = 5, s = 0;
  using edge = Edge<int>;
  vector<vector<edge>> g(n);
  g[0].emplace_back(edge{1, 1});
  g[0].emplace_back(edge{2, 4});
  g[1].emplace_back(edge{2, 2});
  g[2].emplace_back(edge{3, 1});
  g[1].emplace_back(edge{3, 5});
  /*
    0-->1----+
    |   |    |
    |   |    |
    v   |    |
    2<--+    |
    |        v
    +------->3
  */
  auto dist = dijkstra<int>(g, s);
  auto want = vector<int>{0, 1, 3, 4};
  for (int i = 0; i < (int)want.size(); i++) {
    assert(dist[i] == want[i]);
  }
  return 0;
}

// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3139546#1
