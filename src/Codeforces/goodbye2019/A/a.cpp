#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)n; i++)
int main() {

  int tt;
  scanf("%d", &tt);
  while (tt--) {
    int n, k1, k2;
    scanf("%d %d %d", &n, &k1, &k2);
    vector<int> a(k1), b(k2);
    rep(i, k1) { scanf("%d", &a[i]); }
    rep(i, k2) { scanf("%d", &b[i]); }
    bool win = false;
    for (auto e : a) {
      if (e == n) win = true;
    }
    if (win) {
      puts("YES");
    } else {
      puts("NO");
    }
  }
  return 0;
}
