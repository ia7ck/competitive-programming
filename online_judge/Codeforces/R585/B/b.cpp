#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n;
  cin >> n;
  vector<int> a(n);
  for (auto &e : a) {
    cin >> e;
    e /= abs(e);
  }

  int64_t pre = 0, neg = 0;
  rep(i, n) {
    if (a[i] < 0) { pre = i + 1 - pre; }
    neg += pre;
  }
  auto posi = (int64_t)n * (n + 1) / 2 - neg;
  cout << neg << " " << posi << endl;

  return 0;
}
