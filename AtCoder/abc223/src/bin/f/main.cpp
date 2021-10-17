#include<iostream>
#include<vector>
#include "atcoder/lazysegtree"

using namespace std;
#define rep(i, n) for (int (i) = 0; (i) < (n); (i)++)

using ll = long long;

ll mapping(ll l, ll r) {
    return l + r;
}

ll composition(ll l, ll r) {
    return l + r;
}

ll id() {
    return 0;
}

ll op2(ll l, ll r) {
    return min(l, r);
}

ll e2() {
    return (ll) (1e18);
}


int main() {
    int n, q;
    cin >> n >> q;
    string s;
    cin >> s;

    vector<ll> cul(n + 1);
    rep(i, n) {
        if (s[i] == '(') {
            cul[i + 1] = cul[i] + 1;
        } else {
            cul[i + 1] = cul[i] - 1;
        }
    }
    rep(i, n + 1) {
        cerr << cul[i] << " ";
    }
    cerr << endl;

    atcoder::lazy_segtree<ll, op2, e2, ll, mapping, composition, id> seg2(cul);
    while (q--) {
        int type, l, r;
        cin >> type >> l >> r;
        if (type == 1) {
            if (s[l - 1] == '(' && s[r - 1] == ')') {
                s[l - 1] = ')';
                s[r - 1] = '(';
                seg2.apply(l, r, -2);
            } else if (s[l - 1] == ')' && s[r - 1] == '(') {
                s[l - 1] = '(';
                s[r - 1] = ')';
                seg2.apply(l, r, +2);
            }
        } else {
            ll sum = seg2.get(r) - seg2.get(l - 1);
            ll mn = seg2.prod(l, r + 1);
            if (sum == 0 && mn >= seg2.get(r)) {
                cout << "Yes" << endl;
            } else {
                cout << "No" << endl;
            }
        }
    }

    return 0;
}
