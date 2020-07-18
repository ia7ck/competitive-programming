#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int a1, a2, k1, k2, n;
  cin >> a1 >> a2 >> k1 >> k2 >> n;

  auto mn = min(a1 + a2, max(0, n - (k1 - 1) * a1 - (k2 - 1) * a2));
  int mx = 0;
  if (k1 > k2) {
    swap(k1, k2);
    swap(a1, a2);
  }
  while (a1 > 0 and n - k1 >= 0) {
    n -= k1;
    a1 -= 1;
    mx += 1;
  }
  while (a2 > 0 and n - k2 >= 0) {
    n -= k2;
    a2 -= 1;
    mx += 1;
  }
  cout << mn << " " << mx << endl;

  return 0;
}
