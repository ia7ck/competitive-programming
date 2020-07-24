#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n;
  cin >> n;
  vector<int> p(4);
  rep(i, 4) cin >> p[i];
  vector<int> t(4);
  rep(i, 4) cin >> t[i];

  int ans = 1000000000;
  for (int a = 0; a <= n; a++) {
    for (int b = 0; a + b <= n; b++) {
      for (int c = 0; a + b + c <= n; c++) {
        for (int d = 0; a + b + c + d <= n; d++) {
          if (a * t[0] + b * t[1] + c * t[2] + d * t[3] >= n) {
            ans = min(ans, a * p[0] + b * p[1] + c * p[2] + d * p[3]);
          }
        }
      }
    }
  }
  cout << ans << endl;

  return 0;
}
