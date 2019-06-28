#include <iostream>
#include <vector>
#include <algorithm>
#include <tuple>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

void chmax(int64_t &a, int64_t b) {
  if (a < b) a = b;
}

int main () {
  
  int n, m;
  cin >> n >> m;
  int64_t c;
  cin >> c;
  vector<vector<tuple<int, int64_t>>> rights(n + 1);
  rep(i, m) {
    int l, r;
    int64_t p;
    cin >> l >> r >> p;
    rights[r].emplace_back(l - 1, p);
  }
  
  const int64_t inf = 1e18;
  vector<vector<int64_t>> dp(n + 1, vector<int64_t>(2, -inf));
  dp[0][1] = 0;
  for (int x = 1; x <= n; x++) {
    chmax(dp[x][0], max(dp[x - 1][0], dp[x - 1][1]));
    for (auto t: rights[x]) {
      int l;
      int64_t p;
      tie(l, p) = t;
      chmax(dp[x][1], dp[l][1] + p - (x < n ? c : 0));
      chmax(dp[x][1], dp[l][0] + p - c - (x < n ? c : 0));
    }
  }
  
  rep(i, n + 1) rep(j, 2) {
    // cout << dp[i][j] << endl;
  }
  cout << max(dp[n][0], dp[n][1]) << endl;

  return 0;
}
