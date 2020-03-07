#include <cstdint>

template <std::uint_fast64_t Modulus> class modint {
  using u64 = std::uint_fast64_t;

public:
  u64 a;

  constexpr modint(const u64 x = 0) noexcept : a(x % Modulus) {}
  constexpr u64 &value() noexcept { return a; }
  constexpr const u64 &value() const noexcept { return a; }
  constexpr modint operator+(const modint rhs) const noexcept {
    return modint(*this) += rhs;
  }
  constexpr modint operator-(const modint rhs) const noexcept {
    return modint(*this) -= rhs;
  }
  constexpr modint operator*(const modint rhs) const noexcept {
    return modint(*this) *= rhs;
  }
  constexpr modint operator/(const modint rhs) const noexcept {
    return modint(*this) /= rhs;
  }
  constexpr modint &operator+=(const modint rhs) noexcept {
    a += rhs.a;
    if (a >= Modulus) {
      a -= Modulus;
    }
    return *this;
  }
  constexpr modint &operator-=(const modint rhs) noexcept {
    if (a < rhs.a) {
      a += Modulus;
    }
    a -= rhs.a;
    return *this;
  }
  constexpr modint &operator*=(const modint rhs) noexcept {
    a = a * rhs.a % Modulus;
    return *this;
  }
  constexpr modint &operator/=(modint rhs) noexcept {
    u64 exp = Modulus - 2;
    while (exp) {
      if (exp % 2) {
        *this *= rhs;
      }
      rhs *= rhs;
      exp /= 2;
    }
    return *this;
  }
};

// http://noshi91.hatenablog.com/entry/2019/03/31/174006

#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

#define rep(i, n) for (int i = 0; i < (n); i++)

using mint = modint<1000000000 + 7>;
mint solve(int r, int c) {
  auto n = r + c + 2;
  vector<mint> fac(n), inv(n);
  fac[0] = 1;
  for (int i = 1; i < n; i++) {
    fac[i] = fac[i - 1] * i;
  }
  inv[n - 1] = mint(1) / fac[n - 1];
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
    res += countPath(i + 1, c);
  }
  return res;
}

int main() {

  int r1, c1, r2, c2;
  cin >> r1 >> c1 >> r2 >> c2;

  auto ans = solve(r2, c2);
  ans -= solve(r1 - 1, c2);
  ans -= solve(r2, c1 - 1);
  ans += solve(r1 - 1, c1 - 1);
  cout << ans.a << endl;

  return 0;
}
