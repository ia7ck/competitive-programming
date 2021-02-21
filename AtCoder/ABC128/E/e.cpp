#include <algorithm>
#include <cassert>
#include <iostream>
#include <set>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n, q;
  cin >> n >> q;
  struct E {
    int t, val;
    bool add;
  };
  vector<E> events;
  rep(_, n) {
    int s, t, x;
    cin >> s >> t >> x;
    events.push_back(E{s - x, x, true});
    events.push_back(E{t - x, x, false});
  }
  sort(events.begin(), events.end(), [&](const auto &lhs, const auto &rhs) {
    if (lhs.t == rhs.t) {
      if (lhs.add == rhs.add) {
        return lhs.val < rhs.val;
      } else {
        return !lhs.add; // 時刻がかぶったら先に削除を処理する
      }
    } else {
      return lhs.t < rhs.t; // イベントが起きる時刻の早い順
    }
  });
  const int inf = 1e9 + 7;
  vector<int> cand(n * 2, inf);
  set<int> s;
  rep(i, n * 2) {
    const auto &ev = events[i];
    if (ev.add) {
      s.insert(ev.val);
    } else {
      s.erase(ev.val);
    }
    if (s.size() > 0) {     // 工事中がひとつ以上ある
      cand[i] = *s.begin(); // 最左 (原点に近い座標)
    }
  }
  while (q--) {
    int d;
    cin >> d;
    auto it = upper_bound(
        events.begin(), events.end(), E{d, 0, false},
        [&](const auto &lhs, const auto &rhs) { return lhs.t < rhs.t; });
    // events[j - 1].t <= d となる最大のj
    auto j = it - events.begin();
    if (j == 0 or j == n * 2) {
      cout << -1 << endl;
      continue;
    }
    auto ans = cand[j - 1];
    if (ans == inf) ans = -1;
    cout << ans << endl;
  }
  return 0;
}
