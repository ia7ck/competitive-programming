#include <algorithm>
#include <cassert>
#include <iostream>
#include <set>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct SegmentTree {
  int n = 1;
  vector<int64_t> dat;
  SegmentTree(int N) {
    while (n < N) {
      n *= 2;
    }
    dat.resize(n * 2 - 1, 0);
  }
  void add(int i, int64_t x) {
    dat[i += n - 1] += x;
    while (i > 0) {
      i = (i - 1) / 2;
      dat[i] = dat[i * 2 + 1] + dat[i * 2 + 2];
    }
  }
  int64_t sum(int l, int r) { return sum(l, r, 0, 0, n); }
  int64_t sum(int ql, int qr, int i, int il, int ir) {
    if (qr <= il or ir <= ql) return 0;
    if (ql <= il and ir <= qr) return dat[i];
    auto m = (il + ir) / 2;
    return sum(ql, qr, i * 2 + 1, il, m) + sum(ql, qr, i * 2 + 2, m, ir);
  }
};

int main() {

  int n, q;
  cin >> n >> q;
  vector<int> a(n);
  for (auto &e : a) {
    cin >> e;
  }
  SegmentTree s(n);
  rep(i, n) s.add(i, a[i]);
  set<int> tr; // 末尾が連結されていない電車の番号 0-indexed
  rep(i, n) tr.insert(i);
  while (q--) {
    int t, x;
    cin >> t >> x;
    x--;
    if (t == 1) {
      tr.erase(x);
    } else if (t == 2) {
      tr.insert(x);
    } else if (t == 3) {
      s.add(x, 1);
    } else {
      auto it = tr.lower_bound(x);
      if (it == tr.begin()) {
        cout << s.sum(0, *it + 1) << endl;
      } else {
        // 先頭が連結されていない電車を指すための +1
        // [l, r) に合わせるための +1
        cout << s.sum(*prev(it) + 1, *it + 1) << endl;
      }
    }
  }
  return 0;
}
