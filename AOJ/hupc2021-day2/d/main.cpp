#include <cstdio>
#include <algorithm>

using namespace std;

#define REP(i, n) for (int (i) = 0; (i) < (n); (i)++)

int main() {
    int t;
    scanf("%d", &t);
    bool seen[500];
    while (t--) {
        int n, m, v, a, b;
        scanf("%d %d %d %d %d", &n, &m, &v, &a, &b);
        REP(i, m) {
            seen[i] = false;
        }
        v %= m;
        seen[v] = true;
        bool found = false;
        REP(i, min(500, n - 1)) {
            v = (a * v + b) % m;
            if (seen[v]) {
                found = true;
                puts("Yes");
                break;
            }
            seen[v] = true;
        }
        if (!found) {
            puts("No");
        }
    }
}
