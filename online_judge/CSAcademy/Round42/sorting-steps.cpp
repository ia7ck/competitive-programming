#include <algorithm>
#include <iostream>
#include <map>
#include <vector>

using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

struct FenwickTree {
  int n;
  vector<int> dat;
  FenwickTree(int n) : n(n) {
    dat.resize(n + 1, 0); // [1, n]を使う
  }
  void add(int i, int x) {
    if (i <= 0)
      return;
    for (int k = i; k <= n; k += k & (-k))
      dat[k] += x;
  }
  int sum(int i) { // [1, i]の和
    if (i <= 0)
      return 0;
    int s = 0;
    for (int k = i; k > 0; k -= k & (-k))
      s += dat[k];
    return s;
  }
  int sum(int i, int j) { // [i, j]の和 (i<=j)
    if (i > j)
      return 0;
    else
      return sum(j) - sum(i - 1);
  }
};

int main() {

  int n;
  cin >> n;
  vector<int> a(n);
  rep(i, n) cin >> a[i];

  int tot = 0;
  FenwickTree tree(a.size()); // dat[i]:=a[i]を見たか
  vector<pair<int, int>> ai;
  rep(i, n) ai.push_back({a[i], i});
  sort(ai.begin(), ai.end(), [&](const auto &l, const auto &r) {
    if (l.first == r.first)
      return l.second > r.second;
    else
      return l.first > r.first;
  });
  for (auto pr : ai) {
    tot = max(tot, tree.sum(1, pr.second));
    tree.add(pr.second + 1, 1);
  }
  cout << tot + 1 << endl;

  return 0;
}

// 3 7 7 4 3 7
// => 3
