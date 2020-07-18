#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int T;
  cin >> T;
  while (T--) {
    int n, q;
    cin >> n >> q;
    vector<int64_t> v(n);
    for (auto &e : v) {
      cin >> e;
    }
    auto u = v;
    sort(u.begin(), u.end());
    while (q--) {
      int x, y;
      cin >> x >> y;
      x--;
      y--;
      cout << abs(v[y] - v[x]) + y - x << " ";
      int cnt = upper_bound(u.begin(), u.end(), max(v[x], v[y])) -
                lower_bound(u.begin(), u.end(), min(v[x], v[y]));
      cout << cnt << endl;
    }
  }

  return 0;
}
