#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {

  int n;
  cin >> n;
  vector<int64_t> a(n + 1);
  for (int i = 1; i <= n; i++) {
    for (int j = i; j <= n; j += i) {
      a[j] += 1;
    }
  }
  int64_t ans = 0;
  for (int i = 1; i <= n; i++) {
    ans += i * a[i];
  }
  cout << ans << endl;

  return 0;
}
