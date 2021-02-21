#include <algorithm>
#include <iostream>
#include <vector>
#include <numeric>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  vector<vector<int>> vs = {{1, 0, 0}, {0, 1, 0}, {0, 0, 1}, {1, 1, 0},
                            {0, 1, 1}, {1, 0, 1}, {1, 1, 1}};
  int n = vs.size();
  int q;
  cin >> q;
  while (q--) {
    vector<int> a(3);
    rep(i, 3) cin >> a[i];
    sort(a.begin(), a.end());
    vector<int> ord(n);
    iota(ord.begin(), ord.end(), 0);
    int ans = 0;
    do {
      auto b = a;
      int s = 0;
      for (auto i: ord) {
        rep(j, 3) b[j] -= vs[i][j];
        if (any_of(b.begin(), b.end(), [&](auto x) { return x < 0; })) {
          break;
        }
        s += 1;
      }
      ans = max(ans, s);
    }while(next_permutation(ord.begin(), ord.end()));
    cout << ans << endl;
  }

  return 0;
}
