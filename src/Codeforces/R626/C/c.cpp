#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  int n;
  cin >> n;
  string s;
  cin >> s;

  if (count(s.begin(), s.end(), '(') != count(s.begin(), s.end(), ')')) {
    cout << -1 << endl;
    return 0;
  }
  int d = 0, op = 0, cl = 0, ans = 0;
  bool ok = true;
  for (auto c: s) {
    if (c == '(') {
      op += 1;
      d += 1;
    } else {
      cl += 1;
      d -= 1;
    }
    if (d < 0) ok = false;
    if (op == cl) {
      if (not ok) ans += op + cl;
      d = op = cl = 0;
      ok = true;
    }
  }
  cout << ans << endl;

  return 0;
}
