#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n;
  cin >> n;
  vector<int64_t> a(n);
  for (auto &e : a) {
    cin >> e;
  }

  vector<int64_t> ans(n, 0);
  rep(i, 32) {
    vector<int64_t> acc(n, 0);
    rep(j, n) {
      acc[j] += (a[j] >> i) & 1;
      if (j > 0) acc[j] += acc[j - 1];
      if (acc[j] & 1) {
        ans[j] += (j + 1 - acc[j]) * (1LL << i);
      } else {
        ans[j] += acc[j] * (1LL << i);
      }
    }
  }

  for (auto a : ans) {
    cout << a << endl;
  }

  return 0;
}
