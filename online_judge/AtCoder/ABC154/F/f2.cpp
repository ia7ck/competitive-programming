#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

template <int64_t mod>
struct modint {
  int64_t a;
  modint(int64_t a = 0) {
    a %= mod;
    if (a < 0) a += mod;
    this->a = a;
  }
  modint operator+(modint rhs) { return modint(a + rhs.a); }
  modint operator-(modint rhs) { return modint(a - rhs.a); }
  modint operator*(modint rhs) { return modint(a * rhs.a); }
  modint pow(modint base, int64_t exp) {
    auto b = base;
    auto res = modint(1);
    while (exp > 0) {
      if (exp & 1) { res = res * b; }
      b = b * b;
      exp = exp / 2;
    }
    return res;
  }
  modint inv() { return pow(*this, mod - 2); }
  modint operator/(modint rhs) { return (*this) * rhs.inv(); }
};

using mint = modint<1000000000 + 7>;
mint solve(int r, int c) {
  auto n = r + c + 2;
  vector<mint> fac(n), inv(n);
  fac[0] = 1;
  for (int i = 1; i < n; i++) {
    // fac[i] = mint(i) * fac[i - 1];
    fac[i] = fac[i - 1] * i;
  }
  inv[n - 1] = fac[n - 1].inv();
  for (int i = n - 2; i >= 0; i--) {
    inv[i] = inv[i + 1] * (i + 1);
  }
  auto binom = [&](int a, int b) -> mint {
    if (a < 0 or b < 0) return 0;
    if (a < b) return 0;
    return fac[a] * inv[b] * inv[a - b];
  };
  auto countPath = [&](int a, int b) { return binom(a + b, b); };
  auto res = mint(0);
  for (int i = 0; i <= r; i++) {
    res = res + countPath(i + 1, c);
  }
  return res;
}

int main() {

  int r1, c1, r2, c2;
  cin >> r1 >> c1 >> r2 >> c2;

  auto ans = solve(r2, c2);
  ans = ans - solve(r1 - 1, c2);
  ans = ans - solve(r2, c1 - 1);
  ans = ans + solve(r1 - 1, c1 - 1);
  cout << ans.a << endl;

  return 0;
}
