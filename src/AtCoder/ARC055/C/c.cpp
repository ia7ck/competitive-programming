#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (n); i++)
using namespace std;

vector<int> z_algo(const string &s) {
  int n = s.size();
  vector<int> z(n);
  z[0] = n;
  for (int i = 1, j = 0; i < n;) {
    // assert(s.substr(0, j) == s.substr(i, j));
    while (i + j < n and s[j] == s[i + j]) {
      j += 1;
    }
    if (j == 0) {
      i += 1;
      continue;
    }
    z[i] = j;
    int k = 1;
    while (i + k < n and k + z[k] < j) {
      z[i + k] = z[k];
      k += 1;
    }
    i += k;
    j -= k;
  }
  return z;
}

int main() {

  string s;
  cin >> s;

  int n = s.size();
  auto z = z_algo(s);
  reverse(s.begin(), s.end());
  auto rz = z_algo(s);
  reverse(rz.begin(), rz.end());
  int64_t ans = 0;
  for (int i = 3; i < n; i++) { // [0, i), [i, n)
    auto ac_len = n - i;
    // B が空
    if (i <= ac_len) continue;
    // A or C が空
    if (ac_len == 1) continue;
    auto pre_max_len = min({i - 1, ac_len - 1, z[i]}),
         suf_max_len = min(ac_len - 1, rz[i - 1]);
    // A or C が空
    if (pre_max_len == 0 or suf_max_len == 0) continue;
    auto c = pre_max_len + suf_max_len - (ac_len - 1);
    // A にも C にも含められない文字がある
    if (c < 0) continue;
    ans += c;
  }
  cout << ans << endl;

  return 0;
}
