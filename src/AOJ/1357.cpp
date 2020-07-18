#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

double SQ(double x) { return x * x; }

int main() {

  int n;
  cin >> n;
  vector<int> r(n);
  for (auto &e : r) {
    cin >> e;
  }

  vector<double> cx;
  rep(i, n) {
    double min_x = r[i];
    rep(j, cx.size()) {
      double x = cx[j];
      min_x = max(min_x, x + sqrt(SQ(r[i] + r[j]) - SQ(r[i] - r[j])));
    }
    cx.push_back(min_x);
  }

  double ans = 0.0;
  rep(i, cx.size()) { ans = max(ans, cx[i] + r[i]); }
  printf("%.18f\n", ans);

  return 0;
}
