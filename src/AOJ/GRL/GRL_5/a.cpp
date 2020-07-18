#include <algorithm>
#include <functional>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  int n;
  cin >> n;
  struct E {
    int to, cost;
    E(int to, int cost) : to(to), cost(cost) {}
  };
  vector<vector<E>> g(n);
  rep(i, n - 1) {
    int a, b, c;
    cin >> a >> b >> c;
    g[a].emplace_back(b, c);
    g[b].emplace_back(a, c);
  }

  // 0 を根とする
  // dp[i]: 部分木 i の中で i から最も遠い頂点までの距離
  vector<int> dp(n);
  function<int(int, int)> dfs = [&](int i, int p) -> int {
    for (auto e : g[i]) {
      if (e.to != p) { dp[i] = max(dp[i], dfs(e.to, i) + e.cost); }
    }
    return dp[i];
  };
  dfs(0, -1);

  vector<int> dist(n);
  queue<tuple<int, int, int>> q;
  q.emplace(0, -1, 0);
  while (q.size() > 0) {
    int i, p, a;
    tie(i, p, a) = q.front();
    q.pop();
    int c = g[i].size();
    vector<int> le(c + 1);
    rep(o, c) {
      auto j = g[i][o].to;
      le[o + 1] = max(le[o], (j == p ? a : dp[j]) + g[i][o].cost);
    }
    vector<int> ri(c + 1);
    for (int o = c - 1; o >= 0; o--) {
      auto j = g[i][o].to;
      ri[o] = max(ri[o + 1], (j == p ? a : dp[j]) + g[i][o].cost);
    }
    rep(o, c) {
      auto j = g[i][o].to;
      if (j != p) { q.push(make_tuple(j, i, max(le[o], ri[o + 1]))); }
    }
    dist[i] = le[c]; // = ri[0]
  }
  int ans = 0;
  for (auto d : dist) {
    ans = max(ans, d);
  }
  cout << ans << endl;

  return 0;
}
