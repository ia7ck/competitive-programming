#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n;
  cin >> n;
  vector<int64_t> a(n);
  rep(i, n) { cin >> a[i]; }

  sort(a.begin(), a.end());
  int64_t x = 0, y = 0;
  rep(i, n / 2) {
    x += a[i];
    y += a[n - i - 1];
  }
  if (n & 1) y += a[n / 2];
  cout << x * x + y * y << endl;

  return 0;
}
