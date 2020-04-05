#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int n, m;
vector<vector<int>> g;
vector<int64_t> dp;
vector<int64_t> ans;

int64_t dfs(int i, int p) {
  dp[i] = 1;
  for (auto j : g[i]) {
    if (j == p) continue;
    dp[i] = dp[i] * (dfs(j, i) + 1) % m;
  }
  return dp[i];
}

void solve(int i, int p, int64_t a) {
  ans[i] = a % m;
  for (auto j : g[i]) {
    if (j == p) continue;
    ans[i] = ans[i] * (dp[j] + 1) % m;
  }
  int sz = g[i].size();
  vector<int64_t> cul(sz, 1);
  rep(o, sz) {
    if (o > 0) cul[o] = cul[o - 1];
    auto j = g[i][o];
    if (j == p) continue;
    cul[o] = cul[o] * (dp[j] + 1) % m;
  }
  vector<int64_t> luc(sz, 1);
  for (int o = sz - 1; o >= 0; o--) {
    if (o + 1 < sz) luc[o] = luc[o + 1];
    auto j = g[i][o];
    if (j == p) continue;
    luc[o] = luc[o] * (dp[j] + 1) % m;
  }
  rep(o, sz) {
    auto j = g[i][o];
    if (j == p) continue;
    auto b = a;
    if (o - 1 >= 0) b = b * cul[o - 1] % m;
    if (o + 1 < sz) b = b * luc[o + 1] % m;
    solve(j, i, b + 1);
  }
}

int main() {

  cin >> n >> m;
  g.resize(n);
  rep(i, n - 1) {
    int a, b;
    cin >> a >> b;
    g[a - 1].emplace_back(b - 1);
    g[b - 1].emplace_back(a - 1);
  }

  if (n == 1) {
    cout << 1 << endl;
    return 0;
  }
  dp.resize(n);
  dfs(0, -1);
  ans.resize(n);
  solve(0, -1, 1);
  for (auto a : ans) {
    cout << a << endl;
  }
  return 0;
}
