#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <set>
#include <tuple>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n, m;
  cin >> n >> m;
  vector<vector<int>> g(n), h(n);
  vector<pair<int, int>> edges;
  rep(i, m) {
    int a, b;
    cin >> a >> b;
    a -= 1;
    b -= 1;
    g[a].emplace_back(b);
    h[b].emplace_back(a);
    edges.emplace_back(a, b);
  }

  const int inf = 1e9;
  for (auto e : edges) {
    int src, tgt;
    tie(src, tgt) = e;
    swap(src, tgt);
    vector<int> d(n, inf);
    d[src] = 0;
    queue<int> q;
    q.emplace(src);
    while (q.size() > 0) {
      auto cur = q.front();
      q.pop();
      for (auto nxt : g[cur]) {
        if (d[cur] + 1 < d[nxt]) {
          d[nxt] = d[cur] + 1;
          q.emplace(nxt);
        }
      }
    }
    if (d[tgt] == inf) continue;
    set<int> vs = {tgt};
    while (tgt != src) {
      for (auto v : h[tgt]) {
        if (d[v] + 1 == d[tgt]) {
          vs.insert(v);
          tgt = v;
          break;
        }
      }
    }
    vector<int> indeg(n), outdeg(n);
    for (auto ee : edges) {
      if (vs.count(ee.first) and vs.count(ee.second)) {
        indeg[ee.second] += 1;
        outdeg[ee.first] += 1;
      }
    }
    bool ok = true;
    rep(i, n) {
      if (indeg[i] != outdeg[i]) ok = false;
      if (indeg[i] >= 2) ok = false;
    }
    if (ok) {
      cout << vs.size() << endl;
      for (auto v : vs) {
        cout << v + 1 << endl;
      }
      return 0;
    }
  }

  cout << -1 << endl;
  return 0;
}
