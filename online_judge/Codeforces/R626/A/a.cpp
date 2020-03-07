#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

int main() {

  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int q;
  cin >> q;
  while (q--) {
    int n;
    cin >> n;
    vector<int> a(n);
    for (auto &e : a) {
      cin >> e;
    }
    bool found = false;
    rep(i, n) {
      if (a[i] % 2 == 0) {
        cout << 1 << endl;
        cout << i + 1 << endl;
        found = true;
        break;
      }
    }
    if (found) continue;
    rep(i, n) {
      rep(j, i) {
        if ((a[i] + a[j]) % 2 == 0) {
          cout << 2 << endl;
          cout << i + 1 << " " << j + 1 << endl;
          found = true;
          goto heaven;
        }
      }
    }
    heaven:;
    if (found) continue;
    cout << -1 << endl;
  }

  return 0;
}
