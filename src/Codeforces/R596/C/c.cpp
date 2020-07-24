#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int Q;
  scanf("%d", &Q);
  while (Q--) {
    int n, k, d;
    scanf("%d %d %d", &n, &k, &d);
    vector<int> a(n);
    rep(i, n) scanf("%d", &a[i]);
    int ans = n;
    map<int, int> freq;
    rep(i, n) {
      if (i < d) {
        freq[a[i]] += 1;
      } else {
        auto l = a[i - d], r = a[i];
        freq[l] -= 1;
        if (freq[l] == 0) { freq.erase(l); }
        freq[r] += 1;
      }
      if (i >= d - 1) { ans = min(ans, (int)freq.size()); }
    }
    printf("%d\n", ans);
  }
  return 0;
}
