#include <cstdio>
#include <cassert>
#include <algorithm>
#include <vector>
#include "atcoder/segtree"

using namespace std;
#define REP(i, n) for (int i = 0; i < (int)n; i++)

int op(int a, int b) {
    return a + b;
}

int e() {
    return 0;
}

int target;

bool f(int v) {
    return v < target;
}

int main() {
    int n, q;
    scanf("%d%d", &n, &q);
    vector<int> p(n);
    vector<int> points;
    REP(i, n) {
        scanf("%d", &p[i]);
        points.push_back(p[i]);
    }
    vector<vector<int>> queries;
    REP(i, q) {
        int op;
        scanf("%d", &op);
        if (op == 1) {
            int a, x;
            scanf("%d%d", &a, &x);
            queries.push_back({op, a, x});
            points.push_back(x);
        } else if (op == 2) {
            int a;
            scanf("%d", &a);
            queries.push_back({op, a});
        } else {
            int r;
            scanf("%d", &r);
            queries.push_back({op, r});
        }
    }

    sort(points.begin(), points.end());
    points.erase(unique(points.begin(), points.end()), points.end());

    vector<int> id_by_point(points.size(), -1);
    REP(i, n) {
        p[i] = lower_bound(points.begin(), points.end(), p[i]) - points.begin();
        id_by_point[p[i]] = i;
    }

    atcoder::segtree<int, op, e> seg(points.size());
    REP(i, n) {
        seg.set(p[i], 1);
    }
    for (const auto &query: queries) {
        if (query[0] == 1) {
            int a = query[1], x = query[2];
            a -= 1;
            x = lower_bound(points.begin(), points.end(), x) - points.begin();
            assert(id_by_point[p[a]] == a);
            id_by_point[p[a]] = -1;
            seg.set(p[a], 0);
            p[a] = x;
            id_by_point[p[a]] = a;
            seg.set(p[a], 1);
        } else if (query[0] == 2) {
            int a = query[1];
            a -= 1;
            int r = seg.prod(p[a] + 1, points.size());
            printf("%d\n", r + 1);
        } else {
            int r = query[1];
            target = r;
            int pt = seg.min_left<f>(points.size());
            assert(pt >= 1);
            int id = id_by_point[pt - 1];
            assert(id != -1);
            printf("%d\n", id + 1);
        }
    }

    return 0;
}
