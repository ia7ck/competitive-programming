//
// Created by ia7ck on 2020/12/06.
//

#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

#include "atcoder/segtree"
using namespace atcoder;

int64_t op(int64_t a, int64_t b) { return a + b; }
int64_t e() { return 0; }

int main() {

  int n, q;
  scanf("%d %d", &n, &q);
  vector<int64_t> a(n);
  for (auto &e : a) {
    scanf("%lld", &e);
  }
  segtree<int64_t, op, e> seg(a);
  while (q--) {
    int t;
    scanf("%d", &t);
    if (t == 0) {
      int p;
      int64_t x;
      scanf("%d %lld", &p, &x);
      auto y = seg.get(p);
      seg.set(p, y + x);
    } else {
      int l, r;
      scanf("%d %d", &l, &r);
      printf("%lld\n", seg.prod(l, r));
    };
  }
  return 0;
}
