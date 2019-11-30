#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct Monoid {
  int64_t val, len;
  Monoid() {}
  Monoid(int64_t v, int64_t l) : val(v), len(l) {}
  static Monoid ident() { return Monoid(0, 0); }
  static Monoid multiply(Monoid l, Monoid r) {
    return Monoid(l.val + r.val, l.len + r.len);
  }
  bool operator==(const Monoid &rhs) const {
    return val == rhs.val and len == rhs.len;
  }
};

struct Lonoid {
  int64_t x;
  Lonoid() {}
  Lonoid(int64_t x) : x(x) {}
  static Lonoid ident() { return Lonoid(numeric_limits<int>::min()); }
  static Lonoid multiply(Lonoid f, Lonoid g) { // f \circ g
    if (f == Lonoid::ident()) return g;
    return f;
  }
  static Monoid apply(Lonoid f, Monoid m) { // f(m)
    if (f == Lonoid::ident()) return m;
    return Monoid(f.x * m.len, m.len);
  }
  bool operator==(const Lonoid &rhs) const { return x == rhs.x; }
};

template <typename M, typename L>
struct SegmentTree {
  int n;
  M em;
  L el;
  vector<M> dat;
  vector<L> laz;
  SegmentTree(int N, vector<M> a) {
    assert(N == (int)a.size());
    n = 1;
    em = M::ident();
    el = L::ident();
    while (n < N) {
      n *= 2;
    }
    dat.resize(n * 2 - 1, em);
    rep(i, N) dat[i + n - 1] = a[i];
    for (int i = n - 2; i >= 0; i--) {
      dat[i] = M::multiply(dat[i * 2 + 1], dat[i * 2 + 2]);
    }
    laz.resize(n * 2 - 1, el);
  }
  void _update(int i, L f) {
    dat[i] = L::apply(f, dat[i]);
    laz[i] = L::multiply(f, laz[i]);
  }
  void update(int ql, int qr, L f) { update(ql, qr, f, 0, 0, n); }
  void update(int ql, int qr, L f, int i, int il, int ir) {
    if (qr <= il or ir <= ql) return;
    if (ql <= il and ir <= qr) {
      _update(i, f);
    } else {
      auto m = (il + ir) / 2, lch = i * 2 + 1, rch = i * 2 + 2;
      _update(lch, laz[i]);
      _update(rch, laz[i]);
      laz[i] = el;
      update(ql, qr, f, lch, il, m);
      update(ql, qr, f, rch, m, ir);
      dat[i] = M::multiply(dat[lch], dat[rch]);
    }
  }
  M query(int ql, int qr) { return query(ql, qr, 0, 0, n); }
  M query(int ql, int qr, int i, int il, int ir) {
    if (ql <= il and ir <= qr) {
      return dat[i];
    } else if (qr <= il or ir <= ql) {
      return em;
    } else {
      auto m = (il + ir) / 2;
      return L::apply(laz[i], M::multiply(query(ql, qr, i * 2 + 1, il, m),
                                          query(ql, qr, i * 2 + 2, m, ir)));
    }
  }
};

int main() {

  int n, q;
  cin >> n >> q;
  SegmentTree<Monoid, Lonoid> seg(n, vector<Monoid>(n, Monoid(0, 1)));
  while (q--) {
    int t;
    cin >> t;
    if (t == 0) {
      int s, t, x;
      cin >> s >> t >> x;
      seg.update(s, t + 1, Lonoid(x));
    } else {
      int s, t;
      cin >> s >> t;
      cout << seg.query(s, t + 1).val << endl;
    }
  }

  return 0;
}

// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4017282#1
