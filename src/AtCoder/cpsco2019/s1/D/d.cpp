#include <algorithm>
#include <bitset>
#include <iostream>
#include <numeric>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int64_t n;
  cin >> n;

  const int64_t mod = 1000000000 + 7;
  vector<vector<vector<int64_t>>> mat(
      63, vector<vector<int64_t>>(3, vector<int64_t>(3, 0)));
  mat[0] = {{1, 2, 3}, {2, 1, 3}, {1, 1, 2}};
  for (int t = 1; t < 63; t++) {
    rep(i, 3) rep(j, 3) rep(k, 3) {
      (mat[t][i][j] += (mat[t - 1][i][k] * mat[t - 1][k][j]) % mod) %= mod;
    }
  }
  vector<int64_t> v = {1, 1, 0};
  rep(t, 63) {
    if ((n >> t) & 1) {
      vector<int64_t> to = {0, 0, 0};
      rep(i, 3) {
        rep(j, 3) { (to[i] += (mat[t][i][j] * v[j]) % mod) %= mod; }
      }
      v.swap(to);
    }
  }
  int64_t ans = 0;
  for (auto el : v) {
    ans = (ans + el) % mod;
  }
  cout << ans << endl;

  return 0;
}
