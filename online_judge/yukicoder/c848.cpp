#include <iostream>
#include <vector>
#include <algorithm>
#include <queue>
#include <tuple>

using namespace std;
struct Edge{
  int to;
  int64_t cost;
  Edge(int to, int64_t cost): to(to), cost(cost) {}
};

const int64_t inf = 1e18;
vector<int64_t> dijkstra(vector<vector<Edge>> g, int s) {
  int n = g.size();
  vector<int64_t> d(n, inf);
  d[s] = 0;
  priority_queue<pair<int64_t, int>> q;
  q.emplace(0, s);
  while (q.size() > 0) {
    int64_t dist;
    int v;
    tie(dist, v) = q.top();
    q.pop();
    dist *= -1;
    for (const auto &e: g[v]) {
      if (dist + e.cost < d[e.to]) {
        d[e.to] = dist + e.cost;
        q.emplace(d[e.to] * (-1), e.to);
      }
    }
  }
  return d;
}

int main() {
  
  int n, m, p, q;
  int64_t t;
  cin >> n >> m >> p >> q >> t;
  p--;
  q--;
  vector<vector<Edge>> g(n);
  for (int i = 0; i < m; i++) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;
    b--;
    g[a].emplace_back(b, c);
    g[b].emplace_back(a, c);
  }

  auto d0 = dijkstra(g, 0),
       dp = dijkstra(g, p),
       dq = dijkstra(g, q);
  int64_t ans = -1;
  for (int v = 0; v < n; v++) {
    if ((d0[v] + dp[v] + dq[v]) * 2 <= t) {
      ans = max(ans, t);
    }
    for (int u = 0; u < n; u++) {
      auto time = t - d0[v] - d0[u];
      time -= max(dp[v] + dp[u], dq[v] + dq[u]);
      if (time >= 0) {
        ans = max(ans, d0[v] + d0[u] + time);
      }
    } 
  } 

  cout << ans << endl;
  return 0;
}
