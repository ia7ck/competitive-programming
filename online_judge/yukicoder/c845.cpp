#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

void chmax(int &a, int b) {
  if (a < b) a = b;
}

int main() {

  int n, m;
  cin >> n >> m;
  const int inf = 1000000000;
  vector<vector<int>> d(n, vector<int>(n, -1));
  rep(i, n) d[i][i] = 0;
  rep(i, m) {
    int a, b, c;
    cin >> a >> b >> c;
    a--;
    b--;
    chmax(d[a][b], c);
    chmax(d[b][a], c);
  }

  vector<vector<int>> dp(1 << n, vector<int>(n, 0));
  rep(bits, 1 << n) {
    rep(v, n) {
      if (bits >> v & 1) {
        rep(w, n) {
          if (!(bits >> w & 1) and d[v][w] >= 0) {
            chmax(dp[bits ^ (1 << w)][w], dp[bits][v] + d[v][w]);
          }
        }
      }
    }
  }
  int ans = 0;
  rep(bits, 1 << n) {
    rep(v, n) {
      if (bits >> v & 1) { chmax(ans, dp[bits][v]); }
    }
  }
  cout << ans << endl;

  return 0;
}
