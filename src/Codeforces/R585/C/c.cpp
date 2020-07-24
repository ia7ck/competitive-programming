#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  // 8
  // babbaabb
  // abababaa
  // ^^^  ^^^
  //

  int n;
  cin >> n;
  string s, t;
  cin >> s >> t;
  if ((count(s.begin(), s.end(), 'a') + count(t.begin(), t.end(), 'a')) & 1) {
    cout << -1 << endl;
    return 0;
  }
  if ((count(s.begin(), s.end(), 'b') + count(t.begin(), t.end(), 'b')) & 1) {
    cout << -1 << endl;
    return 0;
  }

  vector<int> ab_idx, ba_idx;
  rep(i, n) {
    if (s[i] == 'a' and t[i] == 'b') ab_idx.emplace_back(i);
    if (s[i] == 'b' and t[i] == 'a') ba_idx.emplace_back(i);
  }
  assert((ab_idx.size() + ba_idx.size()) % 2 == 0);
  vector<pair<int, int>> ans;
  while (ab_idx.size() >= 2) {
    auto i = ab_idx.back();
    ab_idx.pop_back();
    auto j = ab_idx.back();
    ab_idx.pop_back();
    ans.emplace_back(i, j);
  }
  while (ba_idx.size() >= 2) {
    auto i = ba_idx.back();
    ba_idx.pop_back();
    auto j = ba_idx.back();
    ba_idx.pop_back();
    ans.emplace_back(i, j);
  }
  if (ab_idx.size() == 1) {
    assert(ba_idx.size() == 1);
    auto i = ab_idx.back(), j = ba_idx.back();
    ans.emplace_back(i, i);
    ans.emplace_back(i, j);
  }
  cout << ans.size() << endl;
  for (auto p : ans) {
    cout << p.first + 1 << " " << p.second + 1 << endl;
  }

  return 0;
}
