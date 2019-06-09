#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

void add(int64_t &a, int64_t b) { a = (a + b) % 1000000007; }

int main() {

  int n, m;
  cin >> n >> m;
  vector<int> ng(n + 1);
  rep(_, m) {
    int a;
    cin >> a;
    ng[a] = true;
  }

  vector<int64_t> dp(n + 1, 0);
  dp[0] = 1;
  rep(i, n) {
    if (i + 1 <= n and (not ng[i + 1])) { add(dp[i + 1], dp[i]); }
    if (i + 2 <= n and (not ng[i + 2])) { add(dp[i + 2], dp[i]); }
  }
  cout << dp[n] << endl;

  return 0;
}
