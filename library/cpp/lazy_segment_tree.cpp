#include <algorithm>
#include <cassert>
#include <functional>
#include <iostream>
#include <queue>
#include <tuple>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

template <typename M, typename L>
struct SegmentTree {
  int n;
  vector<M> dat;
  vector<L> laz;
  M em;
  L el;
  function<M(M, M)> multiply;
  function<L(L, L)> composite;
  function<M(L, M)> apply;
  SegmentTree(int n0, M em, L el, function<M(M, M)> mul, function<L(L, L)> com,
              function<M(L, M)> app)
      : em(em), el(el), multiply(mul), composite(com), apply(app) {
    n = 1;
    while (n < n0) {
      n *= 2;
    }
    dat.resize(n * 2 - 1, em);
    laz.resize(n * 2 - 1, el);
  }
  void set(int i, M a) {
    i += n - 1;
    dat[i] = a;
    while (i > 0) {
      i = (i - 1) / 2;
      dat[i] = multiply(dat[i * 2 + 1], dat[i * 2 + 2]);
    }
  }
  void _update(int i, L f) {
    dat[i] = apply(f, dat[i]);
    laz[i] = composite(f, laz[i]);
  }
  void update(int ql, int qr, L f) { update(ql, qr, f, 0, 0, n); }
  void update(int ql, int qr, L f, int i, int il, int ir) {
    if (qr <= il or ir <= ql) return;
    if (ql <= il and ir <= qr) {
      _update(i, f);
      return;
    }
    auto m = (il + ir) / 2, lch = i * 2 + 1, rch = i * 2 + 2;
    _update(lch, laz[i]);
    _update(rch, laz[i]);
    laz[i] = el;
    update(ql, qr, f, lch, il, m);
    update(ql, qr, f, rch, m, ir);
    dat[i] = multiply(dat[lch], dat[rch]);
  }
  M query(int ql, int qr) { return query(ql, qr, 0, 0, n); }
  M query(int ql, int qr, int i, int il, int ir) {
    if (ql <= il and ir <= qr) return dat[i];
    if (qr <= il or ir <= ql) return em;
    auto m = (il + ir) / 2;
    // f(l + r) = f(l) + f(r)
    return apply(laz[i], multiply(query(ql, qr, i * 2 + 1, il, m),
                                  query(ql, qr, i * 2 + 2, m, ir)));
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
      cout << dat[i] << " ";
      if (i * 2 + 1 < (int)dat.size()) {
        q.emplace(i * 2 + 1, f);
        q.emplace(i * 2 + 2, false);
      }
    }
    cout << endl << "END" << endl;
  }
};

struct M {
  int64_t x, l;
  M(int64_t x, int64_t l) : x(x), l(l) {}
};

ostream &operator<<(ostream &os, M &m) {
  os << m.x;
  return os;
}

int main() {
  // DSL_2_G

  int n, q;
  cin >> n >> q;
  using L = int64_t;
  const L el = 0;
  SegmentTree<M, L> seg(
      n, M(0, 0), el, [&](M a, M b) -> M { return M(a.x + b.x, a.l + b.l); },
      [&](L f, L g) -> L { return f + g; },
      [&](L f, M a) -> M { return f == el ? a : M(a.x + f * a.l, a.l); });
  rep(i, n) { seg.set(i, M(0, 1)); }
  while (q--) {
    int t;
    cin >> t;
    if (t == 0) {
      int s, t, x;
      cin >> s >> t >> x;
      seg.update(s - 1, t, x); // range add
    } else {
      int s, t;
      cin >> s >> t;
      cout << seg.query(s - 1, t).x << endl;
    }
    // seg.tour();
  }

  return 0;
}

