#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int T;
  cin >> T;
  while (T--) {
    int n;
    cin >> n;
    vector<int64_t> a(n);
    for (auto &e : a) {
      cin >> e;
    }
    int64_t x = 0;
    rep(i, 32) {
      int pp = 0;
      for (auto e : a) {
        if (e & (1LL << i)) { pp += 1; }
      }
      if (n - pp < pp) x ^= (1LL << i);
    }
    int64_t sum = 0;
    for (auto e : a) {
      sum += e ^ x;
    }
    cout << sum << endl;
  }
  return 0;
}

/*
  010
  011
  100
  101
  110
*/
