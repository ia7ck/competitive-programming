#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

// g が DAG のとき使える
vector<int> toposo(const vector<vector<int>> &g) {
  int n = g.size();
  vector<int> deg_in(n, 0);
  rep(i, n) for (int j : g[i]) deg_in[j]++;
  queue<int> Q;
  rep(i, n) if (deg_in[i] == 0) Q.push(i);
  vector<int> order;
  while (Q.size() > 0) {
    auto i = Q.front();
    Q.pop();
    order.push_back(i);
    for (int j : g[i]) {
      if ((--deg_in[j]) == 0) Q.push(j);
    }
  }
  return order;
}

int main() {

  const int n = 9, m = 9;
  vector<vector<int>> g(n);
  vector<vector<int>> es = {{0, 2}, {1, 2}, {2, 3}, {3, 4}, {3, 5},
                            {5, 6}, {4, 7}, {6, 7}, {7, 8}};
  assert(es.size() == m);
  for (auto e : es) {
    g[e[0]].emplace_back(e[1]);
  }
  /*
    0-->2-->3--+-->4------+-->7-->8
        ^      |          |
        |      |          |
    1---+      +-->5-->6--+
  */
  auto order = toposo(g);
  vector<bool> seen(n, false);
  for (auto i : order) {
    for (auto e : es) {
      if (e[1] == i) {
        assert(seen[e[0]]); // e[0]: source
      }
    }
    seen[i] = true;
  }
  return 0;
}

// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0519
