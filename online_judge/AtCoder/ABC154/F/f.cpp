#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

const int64_t mod = 1000000000 + 7;
int64_t mpow(int64_t a, int64_t x) {
  int64_t res = 1;
  while (x > 0) {
    if (x & 1) { res = res * a % mod; }
    a = a * a % mod;
    x = x / 2;
  }
  return res;
}

int64_t solve(int r, int c) {
  auto n = r + c + 2;
  vector<int64_t> fac(n), inv(n);
  fac[0] = 1;
  for (int i = 1; i < n; i++) {
    fac[i] = i * fac[i - 1] % mod;
  }
  inv[n - 1] = mpow(fac[n - 1], mod - 2);
  for (int i = n - 2; i >= 0; i--) {
    inv[i] = inv[i + 1] * (i + 1) % mod;
  }
  auto binom = [&](int a, int b) -> int64_t {
    if (a < 0 or b < 0) return 0;
    if (a < b) return 0;
    return fac[a] * inv[b] % mod * inv[a - b] % mod;
  };
  auto countPath = [&](int a, int b) { return binom(a + b, b); };
  int64_t res = 0;
  for (int i = 0; i <= r; i++) {
    res = (res + countPath(i + 1, c)) % mod;
  }
  return res;
}

int main() {

  int r1, c1, r2, c2;
  cin >> r1 >> c1 >> r2 >> c2;

  auto ans = solve(r2, c2);
  ans = (ans - solve(r1 - 1, c2) + mod) % mod;
  ans = (ans - solve(r2, c1 - 1) + mod) % mod;
  ans = (ans + solve(r1 - 1, c1 - 1)) % mod;
  cout << ans << endl;

  return 0;
}
