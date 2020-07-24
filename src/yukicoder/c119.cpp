#include <algorithm>
#include <iostream>
#include <limits>
#include <vector>
using namespace std;
// ford-fulkerson
template <typename T>
class Graph {
  struct Edge {
    int to, rev;
    T cap;
    Edge(int to, int rev, T cap) : to(to), rev(rev), cap(cap) {}
  };
  vector<vector<Edge>> g;
  T inf;
  vector<int> visited;

public:
  Graph(int n) {
    g.resize(n);
    inf = numeric_limits<T>::max() / 3;
    visited.resize(n, false);
  }
  void add_edge(int from, int to, T cap) {
    g[from].emplace_back(to, g[to].size(), cap);
    g[to].emplace_back(from, (int)g[from].size() - 1, 0);
  }
  T dfs(int i, T curf, int sink) {
    if (i == sink) return curf;
    visited[i] = true;
    for (auto &&e : g[i]) {
      if (visited[e.to] or e.cap == 0) continue;
      auto tmpf = dfs(e.to, min(e.cap, curf), sink);
      if (tmpf > 0) {
        e.cap -= tmpf;
        g[e.to][e.rev].cap += tmpf;
        return tmpf;
      }
    }
    return 0; // 流せなかった
  }
  T ford(int source, int sink) {
    T maxf = 0;
    while (true) {
      fill(visited.begin(), visited.end(), false);
      auto f = dfs(source, inf, sink);
      if (f > 0) {
        maxf += f;
      } else {
        break;
      }
    }
    return maxf;
  }
};

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n;
  cin >> n;
  auto S = n * 2, T = S + 1;
  Graph<int> g(n * 2 + 2);
  auto inf = numeric_limits<int>::max() / 3;
  int sum = 0;
  rep(i, n) {
    int b, c;
    cin >> b >> c;
    sum += b + c; // ぜんぶの満足度が得られるとする
    // この辺が切られたら国 i のツアーに参加しないことを意味する
    g.add_edge(S, i, b);
    // 国 i のツアーに参加したら国 i のツアーに参加する (切られない)
    g.add_edge(i, i + n, inf);
    // 切られたら国 i のツアーに参加する
    g.add_edge(i + n, T, c);
    // 両方切られたら国 i で得られる満足度は 0 (国 i に行かない)
  }
  int m;
  cin >> m;
  rep(i, m) {
    int d, e;
    cin >> d >> e;
    // 国 d のツアーに参加したら国 e のツアーに参加する
    // d に参加かつ e に不参加という状況を防ぐために容量が inf
    g.add_edge(d, e + n, inf);
  }
  auto min_cut = g.ford(S, T);
  cout << sum - min_cut << endl;

  return 0;
}
