#include <atcoder/lazysegtree>
#include <iostream>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

int N, D;

struct S {
    vector<int> v;
};

struct F {
    int k;
};

S op(S l, S r) {
    vector<int> v(D);
    REP(i, D) { v[i] = l.v[i] ^ r.v[i]; }
    return S{v};
}

S e() { return S{vector<int>(D, 0)}; }

S mapping(F f, S s) {
    vector<int> v;
    for (int i = f.k; i < D; i++) {
        v.push_back(s.v[i]);
    }
    REP(i, f.k) { v.push_back(s.v[i]); }
    return S{v};
}

F composition(F f, F g) { return F{(f.k + g.k) % D}; }

F id() { return F{0}; }

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    cin >> N >> D;
    vector<int> A(N);
    REP(i, N) cin >> A[i];
    vector<S> a(N);
    int t = 1;
    REP(i, D - 1) { t *= 10; }
    REP(i, N) {
        int x = A[i];
        vector<int> v;
        REP(j, D) {
            v.push_back(x);
            int d = x / t % 10; // 最上位
            x = x % t * 10 + d;
        }
        a[i] = S{v};
    }
    atcoder::lazy_segtree<S, op, e, F, mapping, composition, id> seg(a);
    int Q;
    cin >> Q;
    int K = 0;
    while (Q--) {
        int op;
        cin >> op;
        if (op == 1) {
            int k;
            cin >> k;
            K = (K + k) % N;
        } else if (op == 2) {
            int l, r, k;
            cin >> l >> r >> k;
            l -= 1;
            r -= 1;
            l = (l + K) % N;
            r = (r + K) % N;
            if (l <= r) {
                seg.apply(l, r + 1, F{k});
            } else {
                seg.apply(l, N, F{k});
                seg.apply(0, r + 1, F{k});
            }
        } else {
            int l, r;
            cin >> l >> r;
            l -= 1;
            r -= 1;
            l = (l + K) % N;
            r = (r + K) % N;
            if (l <= r) {
                auto ans = seg.prod(l, r + 1);
                cout << ans.v[0] << endl;
            } else {
                auto ans1 = seg.prod(l, N);
                auto ans2 = seg.prod(0, r + 1);
                cout << (ans1.v[0] ^ ans2.v[0]) << endl;
            }
        }
    }

    return 0;
}
