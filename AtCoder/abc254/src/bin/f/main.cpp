#include <vector>
#include <numeric>
#include "atcoder/segtree"

using namespace std;
#define REP(i, n) for (int (i) = 0; (i) < (n); (i)++)

int op(int l, int r) {
    return gcd(l, r);
}

int e() {
    return 0;
}

int main() {
    int n, q;
    scanf("%d%d", &n, &q);
    vector<int> a(n), b(n);
    REP(i, n) {
        scanf("%d", &a[i]);
    }
    REP(i, n) {
        scanf("%d", &b[i]);
    }

    atcoder::segtree<int, op, e> seg_a(n);
    atcoder::segtree<int, op, e> seg_b(n);
    REP(i, n - 1) {
        seg_a.set(i, abs(a[i + 1] - a[i]));
        seg_b.set(i, abs(b[i + 1] - b[i]));
    }

    REP(qq, q) {
        int h1, h2, w1, w2;
        scanf("%d%d%d%d", &h1, &h2, &w1, &w2);
        auto x = a[h1 - 1] + b[w1 - 1];
        if (h1 < h2) {
            x = gcd(x, seg_a.prod(h1 - 1, h2 - 1));
        }
        if (w1 < w2) {
            x = gcd(x, seg_b.prod(w1 - 1, w2 - 1));
        }
        printf("%d\n", x);
    }
}
