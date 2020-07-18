#include <algorithm>
#include <cassert>
#include <ctype.h>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int rec(vector<string> &a, int i, int j) {
  assert(a[i][j] == '+' or a[i][j] == '*');
  vector<int> targets;
  for (int y = i + 1; y < a.size(); y++) {
    if (a[y].size() <= j) continue;
    if (isdigit(a[y][j])) break;
    if (a[y][j] == '+' or a[y][j] == '*') break;
    if (j + 1 < a[y].size()) {
      auto ch = a[y][j + 1];
      if (isdigit(ch)) {
        targets.push_back(ch - '0');
      } else if (ch == '+' or ch == '*') {
        targets.push_back(rec(a, y, j + 1));
      }
    }
  }
  int res = a[i][j] == '+' ? 0 : 1;
  for (auto t : targets) {
    if (a[i][j] == '+') {
      res += t;
    } else {
      res *= t;
    }
  }
  return res;
}

int main() {

  while (true) {
    int n;
    cin >> n;
    if (n == 0) break;

    if (n == 1) {
      int d;
      cin >> d;
      cout << d << endl;
      continue;
    }
    vector<string> a(n);
    rep(i, n) cin >> a[i];
    auto res = rec(a, 0, 0);
    cout << res << endl;
  }

  return 0;
}
