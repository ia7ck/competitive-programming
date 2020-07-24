#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

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
  SegmentTree(vector<M> a) {
    int N = a.size();
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

vector<int64_t> solve(vector<int64_t> &a) {
  int n = a.size();
  vector<Monoid> b;
  for (auto e : a) {
    b.emplace_back(e, 1);
  }
  SegmentTree<Monoid, Lonoid> seg(b);
  vector<int64_t> le(n);
  le[0] = 0;
  for (int i = 1; i < n; i++) {
    auto prev = seg.query(i - 1, i).val;
    if (prev > a[i]) {
      auto ng = -1, ok = i - 1; // a[ng] <= a[i], a[ok] > a[i]
      while (ok - ng > 1) {
        auto m = (ok + ng) / 2;
        if (seg.query(m, m + 1).val > a[i]) {
          ok = m;
        } else {
          ng = m;
        }
      }
      seg.update(ok, i, Lonoid(a[i]));
    }
    le[i] = seg.query(0, i).val;
  }
  return le;
}

vector<int64_t> construct(vector<int64_t> &a, int i) {
  int n = a.size();
  vector<int64_t> res(n);
  res[i] = a[i];
  auto h = a[i];
  for (int j = i - 1; j >= 0; j--) {
    if (a[j] <= h) { h = a[j]; }
    res[j] = h;
  }
  h = a[i];
  for (int k = i + 1; k < n; k++) {
    if (h >= a[k]) { h = a[k]; }
    res[k] = h;
  }
  return res;
}

int main() {

  int n;
  scanf("%d", &n);
  vector<int64_t> a(n);
  rep(i, n) scanf("%lld", &a[i]);

  auto le = solve(a);
  reverse(a.begin(), a.end());
  auto ri = solve(a);
  reverse(a.begin(), a.end());
  reverse(ri.begin(), ri.end());
  int64_t mx = 0;
  rep(i, n) { mx = max(mx, le[i] + ri[i] + a[i]); }
  int p = 0;
  rep(i, n) {
    if (le[i] + ri[i] + a[i] == mx) {
      p = i;
      break;
    }
  }
  auto ans = construct(a, p);
  rep(i, n) {
    printf("%lld", ans[i]);
    if (i + 1 < n) {
      printf(" ");
    } else {
      puts("");
    }
  }

  return 0;
}
