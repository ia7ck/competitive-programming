#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

vector<int> dy = {-1, 0, 0, 1};
vector<int> dx = {0, -1, 1, 0};
bool check(vector<vector<int>> &a) {
  int n = a.size(), m = a[0].size();
  rep(i, n) {
    rep(j, m) {
      int adj = 0;
      rep(k, 4) {
        auto ni = i + dy[k], nj = j + dx[k];
        if (0 <= ni and ni < n and 0 <= nj and nj < m) {
          if (a[i][j] == a[ni][nj]) adj += 1;
        }
      }
      if (adj >= 2) { return false; }
    }
  }
  return true;
}

int solve(int n, int m) {
  vector<int64_t> fib(200000);
  const int64_t mod = 1e9 + 7;
  fib[0] = 2;
  fib[1] = 2;
  for (int i = 2; i < 200000; i++) {
    fib[i] = (fib[i - 1] + fib[i - 2]) % mod;
  }
  if (n > m) swap(n, m);
  auto ans = fib[m];
  rep(i, n - 1) ans = (ans + fib[i]) % mod;
  return ans;
}

int main() {

  if (0) {
    for (int n = 1; n < 6; n++) {
      for (int m = 1; m < 6; m++) {
        int ans = 0;
        rep(bits, 1 << (n * m)) {
          vector<vector<int>> a(n, vector<int>(m, 0));
          rep(i, n) {
            rep(j, m) { a[i][j] = (bits >> (i * m + j)) & 1; }
          }
          if (check(a)) { ans += 1; }
        }
        if (ans != solve(n, m)) {
          cout << n << " " << m << endl;
          cout << "want: " << ans << endl;
          cout << "got: " << solve(n, m) << endl;
        }
      }
    }
  }

  int n, m;
  cin >> n >> m;
  cout << solve(n, m) << endl;

  return 0;
}
