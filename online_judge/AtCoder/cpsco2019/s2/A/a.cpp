#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int m, n;
  cin >> m >> n;

  int s = 0;
  for (int i = 1; i <= n - 1; i++) {
    s += m / n;
  }

  cout << max(0, m - s) << endl;
  return 0;
}
