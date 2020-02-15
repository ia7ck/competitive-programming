#include <algorithm>
#include <iostream>
#include <vector>
#include <tuple>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  int q;
  cin >> q;
  while (q--) {
    int n;
    int64_t m;
    scanf("%d %lld", &n, &m);
    vector<tuple<int64_t, int64_t, int64_t>> v;
    rep(i, n) {
      int64_t t, l, h;
      scanf("%lld %lld %lld", &t, &l, &h);
      v.emplace_back(t, l, h);
    }
    bool ok = true;
    int64_t pt = 0, pl = m, ph = m;
    rep(i, n) {
      int64_t t, l, h;
      tie(t, l, h) = v[i];
      auto dt = t - pt;
      if (ph + dt < l or h < pl - dt) {
        ok = false;
        break;
      }
      pl = max(pl - dt, l);
      ph = min(ph + dt, h);
      pt = t;
    }
    if (ok) {
      puts("YES");
    } else {
      puts("NO");
    }
  }

  return 0;
}
