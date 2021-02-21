#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct Monoid {
  int64_t val;
  Monoid(){};
  Monoid(int64_t v) : val(v){};
  static Monoid ident() { return Monoid(0); }
  static Monoid multiply(Monoid l, Monoid r) { return Monoid(l.val + r.val); }
};

template <typename M>
struct SegmentTree {
  int n;
  M e;
  vector<M> dat;
  SegmentTree(int N, vector<M> a) {
    assert(N == (int)a.size());
    n = 1;
    e = M::ident();
    while (n < N) {
      n *= 2;
    }
    dat.resize(n * 2 - 1, e);
    rep(i, N) dat[i + n - 1] = a[i];
    for (int i = n - 2; i >= 0; i--) {
      dat[i] = M::multiply(dat[i * 2 + 1], dat[i * 2 + 2]);
    }
  }
  M get(int i) { return dat[i + n - 1]; }
  void set(int i, M m) {
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

  int n, q;
  scanf("%d %d", &n, &q);
  vector<Monoid> a;
  SegmentTree<Monoid> seg(n, vector<Monoid>(n, Monoid(0)));
  while (q--) {
    int t;
    scanf("%d", &t);
    if (t == 0) {
      int i, x;
      scanf("%d %d", &i, &x);
      seg.set(i - 1, Monoid(seg.get(i - 1).val + x));
    } else {
      int l, r;
      scanf("%d %d", &l, &r);
      printf("%ld\n", seg.query(l - 1, r).val);
    }
  }

  return 0;
}

// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4017259#1
