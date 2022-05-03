#include <iostream>
#include <ext/pb_ds/assoc_container.hpp>
#include <ext/pb_ds/tree_policy.hpp>
using namespace std;

#define REP(i, n) for (int i = 0; (i) < (int)(n); (i)++)

int main() {

    int n, q;
    cin >> n >> q;
    using P = pair<string, int>;
    __gnu_pbds::tree<P, __gnu_pbds::null_type, less<P>, __gnu_pbds::rb_tree_tag, __gnu_pbds::tree_order_statistics_node_update> set;
    REP(i, n) {
        string s;
        cin >> s;
        set.insert({s, i});
    }
    REP(i, q) {
        int x;
        string t;
        cin >> x >> t;
        auto itr = set.find_by_order(x - 1);
        int k = itr->second;
        set.erase(itr);
        set.insert({t, k});
    }
    
    vector<string> ans(n);
    REP(i, n) {
        auto itr = set.find_by_order(i);
        ans[itr->second] = itr->first;
    }

    for (auto s: ans) {
        cout << s << endl;
    }

    return 0;
}
