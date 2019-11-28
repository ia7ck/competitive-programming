#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct StCoCo {
  vector<vector<int>> g, rev_g, h;
  vector<int> seen, ord, root;

  StCoCo(vector<vector<int>> &g) : g(g) {
    auto n = g.size();
    rev_g.resize(n);
    seen.resize(n, false);
    root.resize(n, -1);
  }
  int operator[](int i) { return root[i]; }
  void dfs(int i) {
    if (not seen[i]) {
      seen[i] = true;
      for (auto j : g[i]) {
        dfs(j);
      }
      ord.push_back(i);
    }
  }
  void rev_dfs(int i, int rt) {
    if (root[i] < 0) {
      root[i] = rt;
      for (auto j : rev_g[i]) {
        rev_dfs(j, rt);
      }
    }
  }
  vector<vector<int>> build() {
    // Kosaraju
    rep(i, g.size()) for (int j : g[i]) rev_g[j].push_back(i); // 逆辺を張る
    rep(i, g.size()) dfs(i);
    reverse(ord.begin(), ord.end());
    int sz = 0; // 圧縮後のグラフの頂点数
    for (auto i : ord) {
      if (root[i] < 0) rev_dfs(i, sz++);
    }
    h.resize(sz);
    rep(i, g.size()) for (auto j : g[i]) {
      int _i = root[i], _j = root[j];    // 連結成分の代表元
      if (_i != _j) h[_i].push_back(_j); // 多重辺もありうる
    }
    return h;
  }
};

int main() {
  const int n = 5, m = 6;
  vector<vector<int>> g(n);
  vector<vector<int>> es = {{0, 1}, {1, 0}, {1, 2}, {2, 4}, {4, 3}, {3, 2}};
  assert(es.size() == m);
  for (auto e : es) {
    g[e[0]].emplace_back(e[1]);
  }
  /*
    0<-->1-->2-->4-->3
             ^       |
             |       |
             +-------+
  */
  StCoCo scc(g);
  scc.build();
  vector<vector<int>> qs = {{0, 1}, {0, 3}, {2, 3}, {3, 4}};
  vector<bool> want = {true, false, true, true};
  for (int i = 0; i < (int)qs.size(); i++) {
    assert((scc[qs[i][0]] == scc[qs[i][1]]) == want[i]);
  }
  return 0;
}

// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4012861#1
