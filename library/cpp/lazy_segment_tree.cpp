#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct Monoid {
  int64_t val;
  Monoid() {}
  Monoid(int64_t v) : val(v) {}
  static Monoid ident() { return Monoid(numeric_limits<int>::max()); }
  static Monoid multiply(Monoid l, Monoid r) {
    return Monoid(min(l.val, r.val));
  }
  bool operator==(const Monoid &rhs) const { return val == rhs.val; }
};

struct Lonoid {
  int64_t x;
  Lonoid() {}
  Lonoid(int64_t x) : x(x) {}
  static Lonoid ident() { return Lonoid(-1); }
  static Lonoid multiply(Lonoid f, Lonoid g) { // f \circ g
    if (f == Lonoid::ident()) return g;
    return f;
  }
  static Monoid apply(Lonoid f, Monoid m) { // f(m)
    if (f == Lonoid::ident()) return m;
    return Monoid(f.x);
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
  SegmentTree(int n0, vector<M> a) {
    assert(n0 == (int)a.size());
    n = 1;
    em = M::ident();
    el = L::ident();
    while (n < n0) {
      n *= 2;
    }
    dat.resize(n * 2 - 1, em);
    rep(i, n0) dat[i + n - 1] = a[i];
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
      // f(l + r) = f(l) + f(r)
      return L::apply(laz[i], M::multiply(query(ql, qr, i * 2 + 1, il, m),
                                          query(ql, qr, i * 2 + 2, m, ir)));
    }
  }
  void tour() {
    cout << endl << "BEGIN";
    queue<pair<int, bool>> q;
    q.emplace(0, true);
    while (q.size() > 0) {
      int i;
      bool f;
      tie(i, f) = q.front();
      q.pop();
      if (f) cout << endl;
      cout << dat[i].val << " ";
      if (i * 2 + 1 < (int)dat.size()) {
        q.emplace(i * 2 + 1, f);
        q.emplace(i * 2 + 2, false);
      }
    }
    cout << endl << "END" << endl;
  }
};

int main() {

  int n, q;
  cin >> n >> q;
  SegmentTree<Monoid, Lonoid> seg(n,
                                  vector<Monoid>(n, Monoid((1LL << 31) - 1)));
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

// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4017266#1
// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4017276#1
// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4017280#1
// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4017282#1
