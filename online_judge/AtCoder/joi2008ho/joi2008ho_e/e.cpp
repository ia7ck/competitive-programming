#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

vector<int> dx = {0, 0, 1, -1}, dy = {1, -1, 0, 0};
void dfs(vector<vector<int>> &a, int x, int y) {
  a[x][y] = -1;
  rep(k, 4) {
    auto nx = x + dx[k], ny = y + dy[k];
    if (0 <= nx and nx < a.size() and 0 <= ny and ny < a[0].size()) {
      if (a[nx][ny] == 0) { dfs(a, nx, ny); }
    }
  }
}

int main() {

  int w, h;
  cin >> w >> h;
  int n;
  cin >> n;
  using P = tuple<int, int, int, int>;
  vector<P> pts;
  vector<int> xs, ys;
  rep(_, n) {
    int x1, y1, x2, y2;
    cin >> x1 >> y1 >> x2 >> y2;
    pts.emplace_back(x1, y1, x2, y2);
    xs.push_back(x1);
    xs.push_back(x2);
    ys.push_back(y1);
    ys.push_back(y2);
  }
  xs.push_back(0);
  xs.push_back(w);
  ys.push_back(0);
  ys.push_back(h);

  sort(xs.begin(), xs.end());
  xs.erase(unique(xs.begin(), xs.end()), xs.end());
  sort(ys.begin(), ys.end());
  ys.erase(unique(ys.begin(), ys.end()), ys.end());
  int W = xs.size() - 1, H = ys.size() - 1;
  vector<vector<int>> a(W, vector<int>(H, 0));
  for (auto p : pts) {
    int x1, y1, x2, y2;
    tie(x1, y1, x2, y2) = p;
    int mx1 = lower_bound(xs.begin(), xs.end(), x1) - xs.begin(),
        my1 = lower_bound(ys.begin(), ys.end(), y1) - ys.begin(),
        mx2 = lower_bound(xs.begin(), xs.end(), x2) - xs.begin(),
        my2 = lower_bound(ys.begin(), ys.end(), y2) - ys.begin();
    a[mx1][my1] += 1;
    if (mx2 < W) a[mx2][my1] -= 1;
    if (my2 < H) a[mx1][my2] -= 1;
    if (mx2 < W and my2 < H) a[mx2][my2] += 1;
  }
  rep(i, W) rep(j, H - 1) a[i][j + 1] += a[i][j];
  rep(j, H) rep(i, W - 1) a[i + 1][j] += a[i][j];

  int cnt = 0;
  rep(x, W) rep(y, H) {
    if (a[x][y] == 0) {
      dfs(a, x, y);
      cnt += 1;
    }
  }
  cout << cnt << endl;

  return 0;
}
