#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

struct StronglyConnectedComponents {
  vector<vector<int>> g, rev_g;
  vector<int> seen, ord, root;
  StronglyConnectedComponents(vector<vector<int>> &g) : g(g) {
    int n = g.size();
    rev_g.resize(n);
    seen.resize(n, false);
    root.resize(n, -1);
    rep(i, n) for (int j : g[i]) rev_g[j].push_back(i); // 逆辺を張る
  }
  void dfs(int i) {
    if (not seen[i]) {
      seen[i] = true;
      for (int j : g[i])
        dfs(j);
      ord.push_back(i);
    }
  }
  void rev_dfs(int i, int rt) {
    if (root[i] < 0) {
      root[i] = rt;
      for (int j : rev_g[i])
        rev_dfs(j, rt);
    }
  }
  vector<vector<int>> contract() {
    rep(i, g.size()) dfs(i);
    reverse(ord.begin(), ord.end());
    int sz = 0; // 圧縮後のグラフの頂点数
    for (int i : ord)
      if (root[i] < 0) rev_dfs(i, sz++);
    vector<vector<int>> h(sz);
    rep(i, g.size()) for (int j : g[i]) {
      int _i = root[i], _j = root[j];    // 連結成分の代表元
      if (_i != _j) h[_i].push_back(_j); // 多重辺もありうる
    }
    return h;
  }
};

int main() {

  int n, m, s;
  cin >> n >> m >> s;
  s--;
  vector<vector<int>> g(n);
  rep(_, m) {
    int a, b;
    cin >> a >> b;
    g[a - 1].push_back(b - 1);
  }

  StronglyConnectedComponents scc(g);
  auto h = scc.contract();
  vector<int> deg_in(h.size(), 0);
  rep(i, h.size()) for (int j : h[i]) deg_in[j]++;
  int need = 0;
  rep(i, h.size()) {
    if (i == scc.root[s]) continue;
    if (deg_in[i] == 0) need++;
  }
  cout << need << endl;

  return 0;
}
