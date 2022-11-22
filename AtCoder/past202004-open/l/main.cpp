#include <cstdio>
#include <cassert>
#include <vector>

#include "atcoder/segtree"

#define REP(i, n) for (int i = 0; i < (int)n; i++)

using namespace std;

int op(int a, int b) {
    return min(a, b);
}

int e() {
    return 1000000000 + 1;
}

int main() {
    int n, k, d;
    scanf("%d%d%d", &n, &k, &d);
    vector<int> a(n);
    REP(i, n) {
        scanf("%d", &a[i]);
    }

    // n = 6, k = 3, d= 3
    if (n <= (k - 1) * d) {
        puts("-1");
        return 0;
    }

    atcoder::segtree<int, op, e> seg(a);
    vector<int> ans;

    int left = 0;
    REP(i, k) {
        // n = 9, k = 3, d = 3
        // 0, 1, 2, 3, 4, 5, 6, 7, 8
        // ^^^^^^^

        int r = n - (k - 1 - i) * d;
        assert(left < r);
        int y = seg.prod(left, r);
        ans.push_back(y);
        int y_pos = -1;
        for (int j = left; j < r; j++) {
            if (a[j] == y) {
                y_pos = j;
                break;
            }
        }
        assert(y_pos != -1);
        left = y_pos + d;
    }

    REP(i, k) {
        printf("%d", ans[i]);
        if (i + 1 < k) {
            printf(" ");
        }
    }
    printf("\n");

    return 0;
}
