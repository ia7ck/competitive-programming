#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {

  string s, t;
  cin >> s >> t;

  const string hoge = "UNRESTORABLE";
  if (s.size() < t.size()) {
    cout << hoge << endl;
    return 0;
  }

  for (int i = 0; i + t.size() <= s.size(); i++) {
    bool ok = true;
    rep(j, t.size()) { ok &= s[i + j] == t[j]; }
    if (ok) {
      string u = "";
      rep(j, s.size()) {
        if (s[j] == '?')
          u += "a";
        else
          u += s[j];
      }
      cout << u << endl;
      return 0;
    }
  }

  vector<string> cand;
  for (int i = 0; i + t.size() <= s.size(); i++) {
    bool ok = true;
    rep(j, t.size()) {
      if (s[i + j] == '?')
        continue;
      ok &= s[i + j] == t[j];
    }
    if (ok) {
      string u = "";
      rep(j, i) {
        if (s[j] == '?')
          u += "a";
        else
          u += s[j];
      }
      u += t;
      for (int j = i + t.size(); j < s.size(); j++) {
        if (s[j] == '?')
          u += "a";
        else
          u += s[j];
      }
      cand.push_back(u);
    }
  }

  if (cand.size() > 0) {
    sort(cand.begin(), cand.end());
    cout << cand[0] << endl;
  } else {
    cout << hoge << endl;
  }
  return 0;
}
