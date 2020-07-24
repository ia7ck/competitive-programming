#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n, m, k;
  cin >> n >> m >> k;
  vector<int> a(n), b(m);
  for (auto &e : a) {
    cin >> e;
  }
  for (auto &e : b) {
    cin >> e;
  }

  auto f = [](const vector<int> &a) {
    vector<int> v;
    int len = 0;
    for (auto e : a) {
      if (e == 1) {
        len += 1;
      } else {
        if (len > 0) {
          v.emplace_back(len);
          len = 0;
        }
      }
    }
    if (len > 0) { v.emplace_back(len); }
    return v;
  };
  auto la = f(a), lb = f(b);

  auto g = [&](int64_t d, const vector<int> &ls) {
    int64_t res = 0;
    for (auto l : ls) {
      if (l >= d) { res += l - d + 1; }
    }
    return res;
  };
  int64_t ans = 0;
  for (int d = 1; d * d <= k; d++) {
    if (k % d != 0) continue;
    ans += g(d, la) * g(k / d, lb);
    if (d != k / d) { ans += g(k / d, la) * g(d, lb); }
  }
  cout << ans << endl;

  return 0;
}
