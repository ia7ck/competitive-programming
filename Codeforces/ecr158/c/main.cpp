#include <vector>
#include <cstdio>
#include <algorithm>

#define REP(i, n) for (int i = 0; i < (int)(n); i++)

using namespace std;
using ll = long long;

void solve(int n, vector<ll> &a) {
    sort(a.begin(), a.end());
    auto x = a[0];
    vector<ll> ans;
    while (any_of(a.begin(), a.end(), [&](auto y) { return y != a[0]; })) {
        REP(i, n) {
            a[i] = (a[i] + x) / 2;
        }
        ans.push_back(x);
    }
    printf("%d\n", ans.size());
    if (ans.size() <= n) {
        REP(i, ans.size()) {
            printf("%lld", ans[i]);
            if (i + 1 < ans.size()) {
                printf(" ");
            } else {
                printf("\n");
            }
        }
    }
}

int main() {
    int t;
    scanf("%d", &t);
    while (t--) {
        int n;
        scanf("%d", &n);
        vector<ll> a(n);
        REP(i, n) {
            scanf("%lld", &a[i]);
        }
        solve(n, a);
    }
    return 0;
}
