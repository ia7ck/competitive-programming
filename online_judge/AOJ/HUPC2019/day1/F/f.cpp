#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n, k, m;
  cin >> n >> k >> m;

  vector<vector<vector<int>>> dp(
      n, vector<vector<int>>(n * 2 + 1, vector<int>(n * 2 + 1, 0)));
  for (int b = 2; b <= n * 2 and abs(b - 1) <= k; b++) {
    dp[0][1][b] = 1;
  }
  for (int i = 0; i + 1 < n; i++) {
    rep(t, n * 2) {
      rep(b, n * 2) {
        if (dp[i][t][b] > 0) {
          if (abs(b + 1 - t) <= k) {
            for (int c = b + 2;
                 c <= n * 2 and abs(b + 2 - c) <= k and abs(b - c) <= k; c++) {
              dp[i + 1][b + 1][c] += dp[i][t][b];
            }
          }
        }
      }
    }
  }
  rep(t, n * 2 + 1) {
    rep(b, n * 2 + 1) {
      cout << t << " " << b << " " << dp[n - 1][t][b] << endl;
    }
  }
  int ans = 0;
  rep(t, n * 2) {
    if (abs(t - n * 2) <= k) ans += dp[n - 1][t][n * 2];
  }
  cout << ans << endl;

  return 0;
}
