#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)n; i++)
int main() {

  int q;
  cin >> q;
  while (q--) {
    int64_t a, m;
    cin >> a >> m;
    vector<int64_t> d;
    for (int64_t i = 1; i * i <= m; i++) {
      if (m % i == 0) {
        d.emplace_back(i);
        if (m / i != i) d.emplace_back(m / i);
      }
    }
    sort(d.rbegin(), d.rend());
    int n = d.size();
    vector<int64_t> c(n);
    rep(i, n) {
      c[i] = m / d[i];
      rep(j, i) {
        if (d[j] % d[i] == 0) c[i] -= c[j];
      }
    }
    auto g = __gcd(a % m, m);
    auto j = find(d.begin(), d.end(), g) - d.begin();
    cout << c[j] << endl;
  }
  return 0;
}
