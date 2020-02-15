#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  int n, m;
  cin >> n >> m;
  vector<string> strs(n);
  for (auto &s : strs) {
    cin >> s;
  }
  vector<int> seen(n, false);
  string ans = "";
  rep(i, n) {
    const auto s = strs[i];
    for (int j = i + 1; j < n; j++) {
      if (seen[j]) continue;
      const auto t = strs[j];
      bool ok = true;
      rep(k, m) {
        if (s[k] != t[m - k - 1]) ok = false;
      }
      if (ok) {
        seen[i] = true;
        seen[j] = true;
        ans += s;
        break;
      }
    }
  }
  string cent = "";
  rep(i, n) {
    const auto s = strs[i];
    if (not seen[i]) {
      bool ok = true;
      rep(k, m) {
        if (s[k] != s[m - k - 1]) ok = false;
      }
      if (ok) {
        cent = s;
        break;
      }
    }
  }
  string sna = ans;
  reverse(sna.begin(), sna.end());
  auto s = ans + cent + sna;
  cout << s.size() << endl;
  cout << s << endl;

  return 0;
}
