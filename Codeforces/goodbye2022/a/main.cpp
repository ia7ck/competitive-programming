#include <iostream>
#include <vector>
#include <queue>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(const vector<long long> &a, const vector<long long> &b) {
    priority_queue<long long, vector<long long>, greater<long long>> que;
    for (auto &&x: a) {
        que.push(x);
    }
    for (auto &&y: b) {
        que.pop();
        que.push(y);
    }
    long long ans = 0;
    while (que.size() >= 1) {
        ans += que.top();
        que.pop();
    }
    cout << ans << '\n';
}

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);
    
    int t;
    cin >> t;
    while (t--) {
        int n, m;
        cin >> n >> m;
        vector<long long> a(n);
        REP(i, n) {
            cin >> a[i];
        }
        vector<long long> b(m);
        REP(j, m) {
            cin >> b[j];
        }
        solve(a, b);
    }
    
    return 0;
}
