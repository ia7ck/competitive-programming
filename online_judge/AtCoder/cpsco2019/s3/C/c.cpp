#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n;
  cin >> n;
  vector<int> acc(1000000 + 1, 0);
  rep(_, n) {
    int s, t;
    cin >> s >> t;
    acc[s] += 1;
    acc[t] -= 1;
  }
  rep(i, 1000000) acc[i + 1] += acc[i];
  int ans = 0;
  rep(i, 1000000) {
    if (acc[i] == 0 and acc[i + 1] > 0) ans += 1;
  }
  cout << ans << endl;

  return 0;
}
