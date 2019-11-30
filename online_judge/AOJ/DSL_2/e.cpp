#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

struct Lonoid {
  int64_t x;
  Lonoid() {}
  Lonoid(int64_t x) : x(x) {}
  static Lonoid ident() { return Lonoid(0); }
  static Lonoid multiply(Lonoid f, Lonoid g) { return Lonoid(f.x + g.x); }
  bool operator==(const Lonoid &rhs) const { return x == rhs.x; }
};

template <typename L>
struct SegmentTree {
  int n;
  L e;
  vector<L> laz;
  SegmentTree(int n0) {
    n = 1;
    e = L::ident();
    while (n < n0) {
      n *= 2;
    }
    laz.resize(n * 2 - 1, e);
  }
  void _update(int i, L f) { laz[i] = L::multiply(f, laz[i]); }
  void update(int ql, int qr, L f) { update(ql, qr, f, 0, 0, n); }
  void update(int ql, int qr, L f, int i, int il, int ir) {
    if (qr <= il or ir <= ql) return;
    if (ql <= il and ir <= qr) {
      _update(i, f);
    } else {
      auto m = (il + ir) / 2, lch = i * 2 + 1, rch = i * 2 + 2;
      _update(lch, laz[i]);
      _update(rch, laz[i]);
      laz[i] = e;
      update(ql, qr, f, lch, il, m);
      update(ql, qr, f, rch, m, ir);
    }
  }
  L composite(int i) {
    auto f = L::ident();
    for (int k = i + n - 1; k > 0; k = (k - 1) / 2) {
      f = L::multiply(laz[k], f);
    }
    return L::multiply(laz[0], f);
  }
};

int main() {

  int n, q;
  cin >> n >> q;
  SegmentTree<Lonoid> seg(n);
  while (q--) {
    int o;
    cin >> o;
    if (o == 0) {
      int s, t;
      int64_t x;
      cin >> s >> t >> x;
      seg.update(s - 1, t, Lonoid(x));
    } else {
      int i;
      cin >> i;
      auto ans = seg.composite(i - 1).x;
      cout << ans << endl;
    }
  }

  return 0;
}

// http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=4017263#1
