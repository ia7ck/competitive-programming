#include <iostream>
#include <vector>
#include <map>
#include <set>
#include "atcoder/lazysegtree"

using namespace std;
#define REP(i, n) for (int (i) = 0; (i) < (n); (i)++)

long long op(long long l, long long r) {
    return l + r;
}

long long e() {
    return 0;
}

long long mapping(long long l, long long r) {
    return l + r;
}

long long composition(long long l, long long r) {
    return l + r;
}

long long id() {
    return 0;
}

int main() {
    int n, m, q;
    scanf("%d%d%d", &n, &m, &q);
    vector<vector<int>> queries;
    REP(qq, q) {
        int op;
        scanf("%d", &op);
        if (op == 1) {
            int l, r, x;
            scanf("%d%d%d", &l, &r, &x);
            queries.push_back({op, l - 1, r - 1, x});
        } else if (op == 2) {
            int i, x;
            scanf("%d%d", &i, &x);
            queries.push_back({op, i - 1, x});
        } else {
            int i, j;
            scanf("%d%d", &i, &j);
            queries.push_back({op, i - 1, j - 1});
        }
    }

    map<int, vector<int>> row_queries;
    vector<vector<int>> op2_queries(q);
    vector<int> op3_base_x(q);
    for (int t = q - 1; t >= 0; t--) {
        int op = queries[t][0];
        if (op == 3) {
            int i = queries[t][1];
            row_queries[i].push_back(t);
        } else if (op == 2) {
            int i = queries[t][1];
            int x = queries[t][2];
            for (auto q_id: row_queries[i]) {
                op2_queries[t].emplace_back(q_id);
                op3_base_x[q_id] = x;
            }
            row_queries[i].clear();
        }
    }

    vector<long long> sub(q);

    atcoder::lazy_segtree<long long, op, e, long long, mapping, composition, id> seg(m);
    REP(t, q) {
        auto query = queries[t];
        int op = query[0];
        if (op == 1) {
            int l = query[1], r = query[2], x = query[3];
            seg.apply(l, r + 1, x);
        } else if (op == 2) {
            for (auto q_id: op2_queries[t]) {
                int j = queries[q_id][2];
                sub[q_id] = seg.get(j);
            }
        } else {
            int j = query[2];
            long long x = op3_base_x[t];
            long long ans = x + seg.get(j) - sub[t];
            cout << ans << endl;
        }
    }
}
