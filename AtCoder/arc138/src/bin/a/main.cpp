#include <cstdio>
#include <vector>
#include <tuple>
#include <algorithm>
#include "atcoder/segtree"

using namespace std;
#define REP(i, n) for (int (i) = 0; (i) < (int)(n); (i)++)

int op(int a, int b) {
    return max(a, b);
}

int e() {
    return -1;
}

int main() {
    int n, k;
    scanf("%d%d", &n, &k);
    vector<int> a(n);
    REP(i, n) {
        scanf("%d", &a[i]);
    }

    vector<tuple<int, int>> b;
    REP(i, k) {
        b.emplace_back(a[i], i);
    }
    sort(b.begin(), b.end());

    atcoder::segtree<int, op, e> seg_b(k);
    REP(i, k) {
        auto &[v, j] = b[i];
        seg_b.set(i, j);
    }

    int inf = 1000000000;
    auto ans = inf;
    for (int i = k; i < n; i++) {
        int j = lower_bound(b.begin(), b.end(), make_tuple(a[i], 0)) - b.begin();
        if (j == 0) {
            continue;
        }
        auto min_index = seg_b.prod(0, j);
        assert(min_index < k);
        ans = min(ans, (k - min_index - 1) + (i - k) + 1);
    }
    if (ans == inf) {
        puts("-1");
    } else {
        printf("%d", ans);
    }
}
