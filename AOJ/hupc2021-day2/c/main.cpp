#include <iostream>
#include <vector>
#include <queue>

using namespace std;

#define REP(i, n) for (int (i) = 0; (i) < (n); (i)++)

int main() {
    int h, w, n;
    cin >> h >> w >> n;
    int sx, sy, gx, gy;
    cin >> sx >> sy >> gx >> gy;
    vector<vector<int>> imos(h + 1, vector<int>(w + 1, 0));
    REP(i, n) {
        int x, y, k;
        cin >> x >> y >> k;
        imos[x - k - 1][y - k - 1] += 1;
        imos[x - k - 1][y + k] -= 1;
        imos[x + k][y - k - 1] -= 1;
        imos[x + k][y + k] += 1;
    }
    REP(i, h + 1) {
        REP(j, w) {
            imos[i][j + 1] += imos[i][j];
        }
    }
    REP(j, w + 1) {
        REP(i, h) {
            imos[i + 1][j] += imos[i][j];
        }
    }

    REP(i, h) {
        REP(j, w) {
            cout << (imos[i][j] == 0);
        }
        cout << endl;
    }

    vector<int> dy = {-1, 0, 0, 1};
    vector<int> dx = {0, -1, 1, 0};
    vector<vector<bool>> seen(h, vector<bool>(w, false));
    seen[sx - 1][sy - 1] = true;
    queue<pair<int, int>> que;
    que.emplace(sx - 1, sy - 1);
    while (!que.empty()) {
        int x, y;
        tie(x, y) = que.front();
        que.pop();
        REP(k, 4) {
            auto nx = x + dx[k];
            auto ny = y + dy[k];
            if (0 <= nx && nx < h && 0 <= ny && ny < w) {
                if (imos[nx][ny] == 0 && !seen[nx][ny]) {
                    seen[nx][ny] = true;
                    que.emplace(nx, ny);
                }
            }
        }
    }
    auto ans = seen[gx - 1][gy - 1];
    if (ans) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}
