#include <algorithm>
#include <cassert>
#include <iostream>
#include <map>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n, K;
  cin >> n >> K;
  vector<int> a(n);
  for (auto &e : a) {
    cin >> e;
  }
  int ans = 0;
  for (int k = 1; k <= K; k++) {
    for (int d = 1; d <= min(k, n); d++) { // d個取り出す
      auto e = k - d;                      // e個戻す
      if (d < e) continue;
      map<int, int> s;
      int sum = 0;
      int left = d - 1, right = n; // [0, left], [right, n - 1]
      rep(i, d) {
        s[a[i]] += 1;
        sum += a[i];
      }
      {
        int sub = 0, cnt = e;
        for (auto p : s) {
          sub += p.first * min(p.second, cnt);
          cnt -= p.second;
          if (cnt <= 0) break;
        }
        ans = max(ans, sum - sub);
      }
      while (left >= 0) {
        s[a[left]] -= 1;
        sum -= a[left];
        s[a[right - 1]] += 1;
        sum += a[right - 1];
        left--;
        right--;
        int sub = 0, cnt = e;
        for (auto p : s) {
          sub += p.first * min(p.second, cnt);
          cnt -= p.second;
          if (cnt <= 0) break;
        }
        ans = max(ans, sum - sub);
      }
    }
  }
  cout << ans << endl;

  return 0;
}
