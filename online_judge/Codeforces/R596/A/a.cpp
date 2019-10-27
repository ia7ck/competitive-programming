#include <algorithm>
#include <iostream>
#include <vector>

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {

  int da, db;
  cin >> da >> db;

  if (da == db) {
    cout << da << "0";
    cout << " ";
    cout << db << "1" << endl;
    return 0;
  }

  if (da + 1 == db) {
    cout << da << " " << db << endl;
    return 0;
  }

  if (da == 9 and db == 1) {
    cout << 9 << " " << 10 << endl;
    return 0;
  }

  cout << -1 << endl;

  return 0;
}
