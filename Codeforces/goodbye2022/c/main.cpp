#include <iostream>
#include <vector>
#include <cassert>
#include <algorithm>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

bool is_prime(int x) {
    assert(x >= 1);
    if (x == 1) {
        return false;
    }
    if (x == 2) {
        return true;
    }
    for (int y = 2; y * y <= x; y++) {
        if (x % y == 0) {
            return false;
        }
    }
    return true;
}

void solve(int n, const vector<long long> &a) {
    REP(i, n) {
        REP(j, i) {
            if (a[i] == a[j]) {
                puts("NO");
                return;
            }
        }
    }

    bool ok = true;
    // 実際は p <= n/2 で十分
    for (int p = 2; p <= n; p++) {
        if (is_prime(p) == false) {
            continue;
        }
        vector<int> freq(p);
        REP(i, n) {
            freq[a[i] % p] += 1;
        }
        if (*min_element(freq.begin(), freq.end()) >= 2) {
            ok = false;
        }
    }
    if (ok) {
        puts("YES");
    } else {
        puts("NO");
    }
}

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int t;
    cin >> t;
    while (t--) {
        int n;
        cin >> n;
        vector<long long> a(n);
        REP(i, n) { cin >> a[i]; }
        solve(n, a);
    }

    return 0;
}
