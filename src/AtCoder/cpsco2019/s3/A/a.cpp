#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  string s;
  cin >> s;
  for (auto &ch : s) {
    if (ch == 'O') {
      ch = 'A';
    } else if (ch == 'A') {
      ch = 'O';
    }
  }
  cout << s << endl;

  return 0;
}
