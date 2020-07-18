#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  while (true) {
    int n, m, s, g1, g2;
    cin >> n >> m >> s >> g1 >> g2;
    if (n == 0) break;
    const int inf = 100000000;
    vector<vector<int>> d(n, vector<int>(n, inf));
    rep(i, n) d[i][i] = 0;
    rep(i, m) {
      int a, b, c;
      cin >> a >> b >> c;
      d[a - 1][b - 1] = c;
    }
    rep(k, n) rep(i, n) rep(j, n) d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
    auto mn = inf;
    rep(i, n) { mn = min(mn, d[s - 1][i] + d[i][g1 - 1] + d[i][g2 - 1]); }
    cout << mn << endl;
  }

  return 0;
}
