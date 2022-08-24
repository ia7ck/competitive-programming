#include <bitset>
#include <cassert>
#include <iostream>
#include <vector>

#define REP(i, n) for (int (i) = 0; (i) < n; (i)++)

int main() {
  int n;
  std::cin >> n;
  std::vector<std::bitset<3000>> bss(n);
  REP(i, n) {
    std::string s;
    std::cin >> s;
    bss[i] = std::bitset<3000>(s);
  }

  long long ans = 0;
  REP(i, n) {
    REP(j, n) {
      if (i == j) {
        continue;
      }
      if (bss[i].test(n - j - 1)) {
        auto k_count = (bss[i] & bss[j]).count();
        ans += k_count;
      }
    }
  }

  assert(ans % 6 == 0);
  std::cout << ans / 6 << '\n';

  return 0;
}
