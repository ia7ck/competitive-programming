#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    int64_t o1 = 0, e1 = 0;
    rep(i, n) {
      int p;
      cin >> p;
      if (p & 1) {
        o1 += 1;
      } else {
        e1 += 1;
      }
    }
    int m;
    cin >> m;
    int64_t o2 = 0, e2 = 0;
    rep(i, m) {
      int q;
      cin >> q;
      if (q & 1) {
        o2 += 1;
      } else {
        e2 += 1;
      }
    }
    cout << o1 * o2 + e1 * e2 << endl;
  }

  return 0;
}
