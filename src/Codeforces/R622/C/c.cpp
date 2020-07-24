#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  int n;
  cin >> n;
  vector<int64_t> a(n);
  for (auto &e: a) {
    cin >> e;
  }

  int64_t mx = 0;
  vector<int64_t> ans(n);
  rep(i, n) {
    vector<int64_t> b(n);
    b[i] = a[i];
    auto h = a[i];
    auto s = h;
    for (int j = i - 1; j >= 0; j--) {
      if (a[j] <= h) {
        h = a[j];
      }
      s += h;
      b[j] = h;
    }
    h = a[i];
    for (int k = i + 1; k < n; k++) {
      if (h >= a[k]) {
        h = a[k];
      }
      s += h;
      b[k] = h;
    }
    if (mx < s) {
      mx = s;
      ans = b;
    }
  }
  rep(i, n) {
    cout << ans[i] << (i + 1 < n ? ' ' : '\n');
  }

  return 0;
}
