#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct Monoid {
  int64_t val;
  Monoid() {}
  Monoid(int64_t v) : val(v) {}
  static Monoid ident() { return Monoid(0); }
  static Monoid multiply(Monoid l, Monoid r) { return Monoid(l.val + r.val); }
};

template <typename M>
struct SegmentTree {
  int n;
  M e;
  vector<M> dat;
  SegmentTree(int n0, vector<M> a) {
    assert(n0 == (int)a.size());
    n = 1;
    e = M::ident();
    while (n < n0) {
      n *= 2;
    }
    dat.resize(n * 2 - 1, e);
    rep(i, n0) dat[i + n - 1] = a[i];
    for (int i = n - 2; i >= 0; i--) {
      dat[i] = M::multiply(dat[i * 2 + 1], dat[i * 2 + 2]);
    }
  }
  M get(int i) { return dat[i + n - 1]; }
  void update(int i, M m) {
    auto k = i + n - 1;
    dat[k] = m;
    while (k) {
      k = (k - 1) / 2;
      dat[k] = M::multiply(dat[k * 2 + 1], dat[k * 2 + 2]);
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
      return M::multiply(query(ql, qr, i * 2 + 1, il, m),
                         query(ql, qr, i * 2 + 2, m, ir));
    }
  }
};

int main() {

  cin.tie(nullptr);
  ios::sync_with_stdio(false);
  int n, q;
  cin >> n >> q;
  vector<Monoid> a;
  rep(i, n) {
    int64_t v;
    cin >> v;
    a.emplace_back(v);
  }
  SegmentTree<Monoid> seg(n, a);
  while (q--) {
    int t;
    cin >> t;
    if (t == 0) {
      int i, x;
      cin >> i >> x;
      seg.update(i, Monoid(seg.get(i).val + x));
    } else {
      int l, r;
      cin >> l >> r;
      cout << seg.query(l, r).val << endl;
    }
  }

  return 0;
}

// https://judge.yosupo.jp/submission/1734
// https://judge.yosupo.jp/submission/1735
