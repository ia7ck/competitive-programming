#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

struct LoCoAn {
  vector<vector<int>> g;
  vector<vector<int>> par;
  vector<int> dep;
  int L = 1;
  LoCoAn(const vector<vector<int>> &g) : g(g) {
    int n = g.size();
    dep.resize(n);
    while ((1 << L) <= n) {
      L += 1;
    }
    par.resize(n, vector<int>(L, -1));
    dfs(0);
    for (int j = 0; j + 1 < L; j++)
      rep(i, n) {
        if (par[i][j] >= 0) par[i][j + 1] = par[par[i][j]][j];
      }
  }
  void dfs(int i, int p = -1) {
    par[i][0] = p;
    dep[i] = p >= 0 ? dep[p] + 1 : 0;
    for (int j : g[i]) {
      if (j != p) dfs(j, i);
    }
  }
  int lca(int s, int t) {
    if (dep[s] < dep[t]) swap(s, t);
    rep(i, L) if ((dep[s] - dep[t]) >> i & 1) s = par[s][i];
    if (s == t) return s;
    for (int i = L - 1; i >= 0; i--) {
      if (par[s][i] != par[t][i]) {
        s = par[s][i];
        t = par[t][i];
      }
    }
    return par[s][0];
  }
};

int main() {
  const int n = 5;
  vector<vector<int>> g(n);
  vector<vector<int>> es = {{0, 1}, {1, 2}, {1, 3}, {0, 4}};
  for (auto e : es) {
    g[e[0]].emplace_back(e[1]);
    g[e[1]].emplace_back(e[0]);
  }
  LoCoAn lca(g);
  /*
      0
      |\
      1 4
      |\
      2 3
  */
  vector<vector<int>> qs = {{2, 3}, {1, 2}, {1, 4}, {0, 4}, {4, 4}};
  vector<int> want = {1, 1, 0, 0, 4};
  for (int i = 0; i < (int)qs.size(); i++) {
    assert(lca.lca(qs[i][0], qs[i][1]) == want[i]);
  }
  return 0;
}

// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4012853#1
