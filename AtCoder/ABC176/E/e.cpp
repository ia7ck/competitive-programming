#include <iostream>
#include <set>
#include <vector>

using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
  int h, w, m;
  cin >> h >> w >> m;
  struct P {
    int x;
    int y;
    P(int x, int y): x(x), y(y) {}
  };
  vector<P> pts;
  rep(i, m) {
    int y, x;
    cin >> y >> x;
    pts.emplace_back(x, y);
  }
  vector<vector<P>> row(h + 1);
  vector<vector<P>> col(w + 1);
  multiset<int> set;
  for (P p: pts) {
    row[p.y].emplace_back(p);
    col[p.x].emplace_back(p);
  }
  for (int x = 0; x <= w; x++) {
    set.insert(col[x].size());
  }
  int ans = 0;
  for (int y = 0; y <= h; y++) {
    int s = 0;
    for (auto p: row[y]) {
      int size = col[p.x].size();
      s = max(s, size - 1);
      auto itr = set.find(size);
      set.erase(itr);
    }
    s = max(s, *set.rbegin());
    ans = max<int>(ans, s + row[y].size());
    for (auto p: row[y]) {
      set.insert(col[p.x].size());
    }
  }
  cout << ans << endl;

  return 0;
}
