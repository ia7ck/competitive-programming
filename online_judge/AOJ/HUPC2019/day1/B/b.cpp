#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int q;
  cin >> q;

  vector<int> a(100005);
  const int64_t inf = 1e9;
  for (int m = 2; m < (int)a.size(); m++) {
    int64_t p = 1;
    for (int x = 2; x * x <= m; x++) {
      if (m % x == 0) {
        p *= x;
        p = min(p, inf);
        if (m % x != x) {
          p *= (m / x);
          p = min(p, inf);
        }
      }
    }
    if (p >= m * 2) { a[m] += 1; }
  }
  rep(i, a.size() - 1) a[i + 1] += a[i];

  while (q--) {
    int n;
    cin >> n;
    cout << a[n] << endl;
  }

  return 0;
}
