#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  string s;
  cin >> s;

  vector<int> freq(26, -1);
  int f = 0;
  for (auto ch : s) {
    auto d = ch - 'a';
    if (freq[d] < 0) {
      freq[d] = 1;
    } else {
      freq[d] += 1;
    }
    f = max(f, freq[d]);
  }

  bool ng = false;
  for (auto el : freq) {
    if (el < 0) continue;
    if (f != el) {
      ng = true;
      break;
    }
  }

  cout << (ng ? "No" : "Yes") << endl;

  return 0;
}
