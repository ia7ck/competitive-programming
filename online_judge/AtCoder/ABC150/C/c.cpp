#include <algorithm>
#include <cassert>
#include <iostream>
#include <numeric>
#include <vector>
using namespace std;
#define rep(i, n) for (int i = 0; i < (int)n; i++)
int main() {

  int n;
  cin >> n;
  vector<int> p(n);
  for (auto &e : p) {
    cin >> e;
    e--;
  }
  vector<int> q(n);
  for (auto &e : q) {
    cin >> e;
    e--;
  }
  vector<int> v(n);
  iota(v.begin(), v.end(), 0);
  int a = -1, b = -1;
  int o = 0;
  do {
    o += 1;
    bool is_p = true, is_q = true;
    rep(i, n) {
      if (p[i] != v[i]) is_p = false;
      if (q[i] != v[i]) is_q = false;
    }
    if (is_p) a = o;
    if (is_q) b = o;
  } while (next_permutation(v.begin(), v.end()));
  assert(a >= 1 and b >= 1);
  cout << abs(a - b) << endl;

  return 0;
}
