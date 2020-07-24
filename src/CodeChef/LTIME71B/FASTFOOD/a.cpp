#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int T;
  cin >> T;
  while (T--) {
    int n;
    cin >> n;
    vector<int> a(n), b(n);
    for (auto &e : a) {
      cin >> e;
    }
    for (auto &e : b) {
      cin >> e;
    }
    vector<int64_t> acc(n + 1), cca(n + 1);
    rep(i, n) acc[i + 1] = acc[i] + a[i];
    for (int i = n - 1; i >= 0; i--) {
      cca[i] = cca[i + 1] + b[i];
    }
    int64_t mx =
        max(*max_element(a.begin(), a.end()), *max_element(b.begin(), b.end()));
    for (int i = 0; i <= n; i++) {
      mx = max(mx, acc[i] + cca[i]);
    }
    cout << mx << endl;
  }

  return 0;
}
