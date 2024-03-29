#include <iostream>
#include <vector>

using namespace std;

#define REP(i, n) for (int (i) = 0; (i) < (n); (i)++)

int main() {
    int m, n;
    cin >> m >> n;
    vector<vector<char>> a(m, vector<char>(n));
    REP(i, m) {
        REP(j, n) {
            cin >> a[i][j];
        }
    }

    int ans = m * n;
    REP(i, m - 1) {
        int u_o = 0, u_x = 0;
        int d_o = 0, d_x = 0;
        REP(k, m) {
            REP(j, n) {
                if (k <= i) {
                    if (a[k][j] == 'o') {
                        u_o += 1;
                    } else {
                        u_x += 1;
                    }
                } else {
                    if (a[k][j] == 'o') {
                        d_o += 1;
                    } else {
                        d_x += 1;
                    }
                }
            }
        }
        ans = min(ans, u_o + d_x);
        ans = min(ans, u_x + d_o);
    }

    REP(j, n - 1) {
        int l_o = 0, l_x = 0;
        int r_o = 0, r_x = 0;
        REP(k, n) {
            REP(i, m) {
                if (k <= j) {
                    if (a[i][k] == 'o') {
                        l_o += 1;
                    } else {
                        l_x += 1;
                    }
                } else {
                    if (a[i][k] == 'o') {
                        r_o += 1;
                    } else {
                        r_x += 1;
                    }
                }
            }
        }
        ans = min(ans, l_o + r_x);
        ans = min(ans, l_x + r_o);
    }

    cout << ans << endl;

    return 0;
}
