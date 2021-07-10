#include <atcoder/lazysegtree>
#include <cstdio>
#include <numeric>
using namespace atcoder;
using namespace std;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using ll = long long;

struct S {
  ll tmp;
};

struct F {
  ll x;
};

S op(S l, S r) { return S{min(l.tmp, r.tmp)}; }

S e() { return S{(ll)1e18}; }

S mapping(F l, S r) { return S{l.x + r.tmp}; }

F composition(F l, F r) { return F{l.x + r.x}; }

F id() { return F{0}; }

vector<ll> sort_by(vector<int> &order, vector<ll> src) {
  vector<ll> dest;
  dest.reserve(order.size());
  for (auto i : order) {
    dest.push_back(src[i]);
  }
  return dest;
}

void solve(int n, int k, vector<ll> a, vector<ll> t) {
  vector<int> ord(k);
  iota(ord.begin(), ord.end(), 0);
  sort(ord.begin(), ord.end(),
       [&](const auto &lhs, const auto &rhs) { return a[lhs] < a[rhs]; });
  a = sort_by(ord, a);
  t = sort_by(ord, t);
  vector<S> u;
  rep(i, k) { u.push_back(S{t[i] + a[i]}); }
  lazy_segtree<S, op, e, F, mapping, composition, id> seg(u);
  vector<ll> ans;
  for (int i = 1; i <= n; i++) {
    // i-1 -> i
    int j = lower_bound(a.begin(), a.end(), i) - a.begin();
    seg.apply(0, j, F{+1});
    seg.apply(j, k, F{-1});
    auto prod = seg.all_prod();
    ans.push_back(prod.tmp);
  }
  printf("%lld", ans[0]);
  for (int i = 1; i < n; i++) {
    printf(" %lld", ans[i]);
  }
  puts("");
}

int main() {

  int q;
  scanf("%d", &q);
  while (q--) {
    int n, k;
    scanf("%d%d", &n, &k);
    vector<ll> a(k);
    rep(i, k) { scanf("%lld", &a[i]); }
    vector<ll> t(k);
    rep(i, k) { scanf("%lld", &t[i]); }
    solve(n, k, a, t);
  }

  return 0;
}
