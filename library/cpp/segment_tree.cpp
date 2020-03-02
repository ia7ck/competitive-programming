#include <algorithm>
#include <cassert>
#include <functional>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

template <typename M>
struct SegmentTree {
  int n;
  vector<M> dat;
  M e;
  function<M(M, M)> multiply;
  SegmentTree(int n0, M e, function<M(M, M)> f) : e(e), multiply(f) {
    n = 1;
    while (n < n0) {
      n = n * 2;
    }
    dat.resize(n * 2 - 1, e);
  }
  M get(int i) { return dat[i + n - 1]; }
  void update(int i, M m) {
    auto k = i + n - 1;
    dat[k] = m;
    while (k) {
      k = (k - 1) / 2;
      dat[k] = multiply(dat[k * 2 + 1], dat[k * 2 + 2]);
    }
  }
  M query(int ql, int qr) { return query(ql, qr, 0, 0, n); }
  M query(int ql, int qr, int i, int il, int ir) {
    if (ql <= il and ir <= qr) {
      return dat[i];
    } else if (qr <= il or ir <= ql) {
      return e;
    } else {
      auto m = (il + ir) / 2;
      return multiply(query(ql, qr, i * 2 + 1, il, m),
                      query(ql, qr, i * 2 + 2, m, ir));
    }
  }
};

int main() {

  cin.tie(nullptr);
  ios::sync_with_stdio(false);
  int n, q;
  cin >> n >> q;
  using M = int64_t;
  SegmentTree<M> seg(n, 0, [&](M a, M b) -> M { return a + b; });
  rep(i, n) {
    int x;
    cin >> x;
    seg.update(i, x);
  }
  while (q--) {
    int t;
    cin >> t;
    if (t == 0) {
      int i, x;
      cin >> i >> x;
      auto y = seg.get(i);
      seg.update(i, y + x);
    } else {
      int l, r;
      cin >> l >> r;
      cout << seg.query(l, r) << endl;
    }
  }

  return 0;
}

// https://judge.yosupo.jp/submission/4476
// https://judge.yosupo.jp/submission/4478
