#include <algorithm>
#include <iostream>
#include <set>
#include <tuple>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n, q;
  cin >> n >> q;
  vector<tuple<int, int, int>> a; // time, type, label
  // time: イベントが起こる時刻
  // type: 0->label削除, 1->label追加, 2->答え求める
  // label: type=0,1のときX[j], 2のとき人の番号
  rep(_, n) {
    int s, t, x;
    cin >> s >> t >> x;
    a.emplace_back(s - x, 1, x);
    a.emplace_back(t - x, 0, x);
  }
  rep(i, q) {
    int d;
    cin >> d;
    a.emplace_back(d, 2, i);
  }
  sort(a.begin(), a.end()); // time, type の昇順
  // 削除->追加->答え求めるの順
  set<int> setx; // X[j]を管理
  vector<int> ans(q, -1);
  for (auto it : a) {
    int type = get<1>(it), label = get<2>(it);
    if (type == 0) {
      setx.erase(label);
    } else if (type == 1) {
      setx.insert(label);
    } else {
      if (setx.size() > 0) ans[label] = *setx.begin();
    }
  }
  rep(i, q) cout << ans[i] << endl;

  return 0;
}
