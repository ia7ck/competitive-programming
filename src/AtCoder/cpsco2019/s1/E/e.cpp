#include <algorithm>
#include <iostream>
#include <map>
#include <set>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n, q;
  cin >> n >> q;
  vector<int> a(n);
  map<int, int> freq;
  for (auto &e : a) {
    cin >> e;
    freq[e] ^= 1;
  }
  while (q--) {
    int l, r, x;
    cin >> l >> r >> x;
    int ans = 0, cnt = 0;
    vector<int> removes;
    for (auto it = freq.lower_bound(l); it != freq.upper_bound(r); it++) {
      ans ^= (it->first) * (it->second);
      cnt += it->second;
      removes.push_back(it->first);
    }
    cout << ans << endl;
    for (auto rm : removes) {
      freq.erase(rm);
    }
    if (cnt & 1) freq[x] ^= 1;
  }

  return 0;
}
