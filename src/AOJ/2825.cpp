#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  while (true) {
    int n, m;
    cin >> n >> m;
    if (n == 0 and m == 0) break;
    vector<vector<int>> args(m);
    rep(i, m) {
      int s, k;
      cin >> s >> k;
      args[i].push_back(s);
      args[i].push_back(k);
      rep(j, k) {
        int c;
        cin >> c;
        args[i].push_back(c - 1);
      }
    }

    vector<int64_t> sum(n, 0);
    for (auto a : args) {
      for (int i = 2; i < (int)a.size(); i++) {
        sum[a[i]] += a[0];
      }
    }
    int64_t ans = 0;
    rep(i, n) { // 人 i が最低得点
      int64_t s = 0;
      // 人 i しか解けない問題は解く
      for (auto a : args) {
        if (a[1] == 1 and a[2] == i) { s += a[0]; }
      }
      auto tmp = sum[i];
      sum[i] = 0;
      ans = max(ans, *max_element(sum.begin(), sum.end()) - s + 1);
      sum[i] = tmp;
    }
    cout << ans << endl;
  }

  return 0;
}
