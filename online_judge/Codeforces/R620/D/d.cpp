#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

void o(vector<int> &a) {
  int n = a.size();
  rep(i, n) { cout << a[i] << (i + 1 == n ? '\n' : ' '); }
}

int main() {

  ios::sync_with_stdio(false);
  cin.tie(nullptr);
  int q;
  cin >> q;
  while (q--) {
    int n;
    string s;
    cin >> n >> s;
    int x = n;
    vector<int> ans(n, -1);
    rep(i, n) {
      if (i + 1 == n or s[i] == '>') {
        for (int j = i; j >= 0 and ans[j] == -1; j--) {
          ans[j] = x;
          x -= 1;
        }
      }
    }
    o(ans);
    fill(ans.begin(), ans.end(), -1);
    x = 1;
    rep(i, n) {
      if (i + 1 == n or s[i] == '<') {
        for (int j = i; j >= 0 and ans[j] == -1; j--) {
          ans[j] = x;
          x += 1;
        }
      }
    }
    o(ans);
  }

  return 0;
}
