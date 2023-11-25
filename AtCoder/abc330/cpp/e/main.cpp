#include <cstdio>
#include <vector>
#include "atcoder/segtree"

#define REP(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int op(int a, int b) { return min(a, b); }

int e() { return 123456789; }

int main() {
    int n, q;
    scanf("%d%d", &n, &q);
    vector<int> a(n);
    REP(i, n) {
        scanf("%d", &a[i]);
    }

    atcoder::segtree<int, op, e> seg(vector<int>(n + 1, 0));
    for (int x: a) {
        if (x <= n) {
            seg.set(x, seg.get(x) + 1);
        }
    }
    while (q--) {
        int i, x;
        scanf("%d%d", &i, &x);
        i -= 1;
        if (a[i] <= n) {
            seg.set(a[i], seg.get(a[i]) - 1);
        }
        a[i] = x;
        if (a[i] <= n) {
            seg.set(a[i], seg.get(a[i]) + 1);
        }
        int ans = seg.max_right(0, [](int v){ return v >= 1; });
        printf("%d\n", ans);
    }
}
