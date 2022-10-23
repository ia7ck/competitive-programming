#include <iostream>
#include <vector>

#define REP(i, n) for (int i = 0; i < (int)n; i++)
using namespace std;

void solve(int n, const string &s) {
    int q = 0;
    for (auto ch: s) {
        if (ch == 'Q') {
            q += 1;
        } else {
            q = max(0, q - 1);
        }
    }
    if (q == 0) {
        puts("Yes");
    } else {
        puts("No");
    }
}

int main() {
    cin.tie(nullptr);
    ios_base::sync_with_stdio(false);

    int t;
    cin >> t;
    while (t--) {
        int n;
        string s;
        cin >> n >> s;
        solve(n, s);
    }

    return 0;
}
