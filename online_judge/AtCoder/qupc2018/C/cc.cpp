#include <algorithm>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

template <typename T> struct Edge {
  int to;
  T cost;
};

template <typename T> vector<T> dijkstra(vector<vector<Edge<T>>> &g, int s) {
  const auto inf = numeric_limits<T>::max();
  vector<T> dist(g.size(), inf);
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

  int h, w, r;
  cin >> h >> w >> r;
  vector<vector<char>> c(h, vector<char>(w));
  rep(i, h) rep(j, w) cin >> c[i][j];

  using edge = Edge<int>;
  vector<vector<edge>> g(h * w + 1);
  vector<pair<int, int>> dir = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
  rep(i, h) rep(j, w) if (c[i][j] != '#') {
    rep(k, dir.size()) {
      auto ni = i + dir[k].first, nj = j + dir[k].second;
      if (0 <= ni && ni < h && 0 <= nj && nj < w) {
        if (c[ni][nj] != '#') {
          g[i * w + j].push_back(edge{ni * w + nj, 1});
          g[ni * w + nj].push_back(edge{i * w + j, 1});
        }
      }
      if (c[i][j] == '@') g[h * w].push_back(edge{i * w + j, 0});
    }
  }
  auto dist = dijkstra<int>(g, h * w);
  vector<vector<edge>> g2(h * w);
  int s, goal;
  rep(i, h) rep(j, w) {
    if (c[i][j] != '#' && dist[i * w + j] > r) {
      rep(k, dir.size()) {
        auto ni = i + dir[k].first, nj = j + dir[k].second;
        if (0 <= ni && ni < h && 0 <= nj && nj < w) {
          if (c[ni][nj] != '#' && dist[ni * w + nj] > r) {
            g2[i * w + j].push_back(edge{ni * w + nj, 1});
            g2[ni * w + nj].push_back(edge{i * w + j, 1});
          }
        }
      }
    }
    if (c[i][j] == 'S') s = i * w + j;
    if (c[i][j] == 'G') goal = i * w + j;
  }
  auto dist2 = dijkstra<int>(g2, s);
  auto d = dist2[goal];
  cout << ((d < numeric_limits<int>::max()) ? d : -1) << endl;
  return 0;
}
