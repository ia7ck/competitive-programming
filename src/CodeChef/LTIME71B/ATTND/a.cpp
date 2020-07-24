#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int T;
  cin >> T;
  while (T--) {
    int n;
    cin >> n;
    vector<pair<string, string>> v;
    map<string, int> freq;
    rep(_, n) {
      string first, last;
      cin >> first >> last;
      v.push_back({first, last});
      freq[first] += 1;
    }
    for (auto name : v) {
      if (freq[name.first] >= 2) {
        cout << name.first << " " << name.second << endl;
      } else {
        cout << name.first << endl;
      }
    }
  }

  return 0;
}
