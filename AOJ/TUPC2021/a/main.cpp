#include<iostream>
#include<vector>
#include<algorithm>
#include<set>

using namespace std;
#define REP(i, n) for (int (i) = 0; (i) < (int)(n); (i)++)

int main() {
    int m;
    cin >> m;
    string s1, s2;
    cin >> s1 >> s2;
    vector<pair<string, string>> tt;
    REP(_, m) {
        string t1, t2;
        cin >> t1 >> t2;
        tt.emplace_back(t1, t2);
    }

    set<char> s1_set(s1.begin(), s1.end());
    set<char> s2_set(s2.begin(), s2.end());

    vector<set<char>> t1_set;
    vector<set<char>> t2_set;
    for (auto &[t1, t2]: tt) {
        t1_set.emplace_back(t1.begin(), t1.end());
        t2_set.emplace_back(t2.begin(), t2.end());
    }

    vector<string> nicknames;
    for (char c1 = 'a'; c1 <= 'z'; c1++) {
        for (char c2 = 'a'; c2 <= 'z'; c2++) {
            if (s1_set.count(c1) == 0 || s2_set.count(c2) == 0) {
                continue;
            }
            bool ok = true;
            REP(i, m) {
                if (t1_set[i].count(c1) >= 1 && t2_set[i].count(c2) >= 1) {
                    ok = false;
                    break;
                }
            }
            if (ok) {
                nicknames.push_back({c1, c2});
            }
        }
    }

    if (nicknames.empty()) {
        cout << "No" << endl;
    } else {
        auto ans = *min_element(nicknames.begin(), nicknames.end());
        cout << "Yes" << endl;
        cout << ans << endl;
    }

    return 0;
}
