#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int n;
  cin >> n;
  string s;
  cin >> s;

  vector<int> acc_l(n + 1, 0), acc_r(n + 1, 0);
  vector<int> pos_l, pos_r;
  rep(i, n) {
    acc_l[i + 1] = acc_l[i] + (s[i] == '<' ? 1 : 0);
    acc_r[i + 1] = acc_r[i] + (s[i] == '>' ? 1 : 0);
    if (s[i] == '<') pos_l.push_back(i);
    if (s[i] == '>') pos_r.push_back(i);
  }

  return 0;
}
