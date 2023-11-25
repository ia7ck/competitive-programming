#include <vector>
#include <cstdio>

#define REP(i, n) for (int i = 0; i < (int)(n); i++)

using namespace std;

void solve(int n, int x, vector<int> &a) {
    a.insert(a.begin(), 0);
    a.push_back(x);
    int ng = 0, ok = 1000;
    while (ok - ng > 1) {
        auto mid = (ok + ng) / 2;
        auto rest = mid;
        bool go = true;
        for (int i = 1; i < a.size(); i++) {
            auto d = a[i] - a[i - 1];
            if (rest < d) {
                go = false;
                break;
            }
            rest -= d;
            if (a[i] < x) {
                rest = mid;
            }
        }
        bool back = true;
        if (go) {
            for (int i = a.size() - 1; i >= 1; i--) {
                auto d = a[i] - a[i - 1];
                if (rest < d) {
                    back = false;
                    break;
                }
                rest -= d;
                if (a[i] > 0) {
                    rest = mid;
                }
            }
        }
        if (go && back) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    printf("%d\n", ok);
}

int main() {
    int t;
    scanf("%d", &t);
    while (t--) {
        int n, x;
        scanf("%d%d", &n, &x);
        vector<int> a(n);
        REP(i, n) {
            scanf("%d", &a[i]);
        }
        solve(n, x, a);
    }
    return 0;
}
