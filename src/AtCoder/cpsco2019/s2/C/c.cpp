#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n, k;
  cin >> n >> k;
  string s;
  cin >> s;

  vector<int> d;
  int cur = 0;
  for (auto ch : s) {
    if (ch == '(') {
      cur += 1;
    } else {
      cur -= 1;
    }
    d.push_back(cur);
  }
  sort(d.rbegin(), d.rend());
  int64_t ans = 0;
  rep(i, k) ans += d[i];
  cout << ans << endl;

  return 0;
}
