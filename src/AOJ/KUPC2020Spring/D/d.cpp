#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  int n, x;
  cin >> n >> x;

  const int m = 1 << 9; // 512
  const int mod = 998244353;
  vector<vector<int>> dp(m, vector<int>(x + 1));
  rep(k, x + 1) dp[0][k] = 1;
  rep(i, n) {
    vector<vector<int>> nxt(m, vector<int>(x + 1));
    rep(j, m) {
      for (int k = 0; k <= x; k++) {
        if ((j ^ k) < m) { nxt[j ^ k][k] += dp[j][k]; }
      }
    }
    rep(j, m) {
      rep(k, x) {
        nxt[j][k + 1] += nxt[j][k];
        nxt[j][k + 1] %= mod;
      }
    }

    dp.swap(nxt);
  }

  cout << dp[x][x] << endl;

  return 0;
}
