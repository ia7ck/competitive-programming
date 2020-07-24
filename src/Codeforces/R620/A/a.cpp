#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  int n;
  cin >> n;
  while (n--) {
    int64_t x, y, a, b;
    cin >> x >> y >> a >> b;
    auto d = y - x, c = a + b;
    if (d % c != 0) {
      cout << -1 << endl;
    } else {
      cout << d / c << endl;
    }
  }

  return 0;
}
