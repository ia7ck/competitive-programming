#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

bool is_prime(int64_t x) {
  for (int64_t y = 2; y * y <= x; y++) {
    if (x % y == 0) return false;
  }
  return true;
}

int main() {

  int n;
  cin >> n;
  vector<int64_t> primes;
  for (int64_t x = 2; x <= n; x++) {
    if (is_prime(x)) primes.push_back(x);
  }
  vector<int> exists(n + 1, false);
  for (auto p : primes) {
    exists[p] = true;
  }
  int ans = 0;
  for (auto r : primes) {
    if (r * r > n * 2) break;
    for (auto p : primes) {
      auto q = r * r - p;
      if (q >= 0 and q <= n and exists[q]) { ans += 1; }
    }
  }
  cout << ans << endl;

  return 0;
}
