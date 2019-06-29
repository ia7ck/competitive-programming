#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  string s;
  cin >> s;
  sort(s.begin(), s.end());

  auto res = s[0] == s[1] and s[2] == s[3] and s[1] != s[2];
  cout << (res ? "Yes" : "No") << endl;

  return 0;
}
