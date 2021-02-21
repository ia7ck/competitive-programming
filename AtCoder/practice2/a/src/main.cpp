//
// Created by ia7ck on 2020/12/06.
//

#include <algorithm>
#include <iostream>
#include <vector>

#include "atcoder/dsu"

using namespace std;
using namespace atcoder;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

int main() {
  int n, q;
  scanf("%d %d", &n, &q);
  dsu ds(n);
  while (q--) {
    int t, u, v;
    scanf("%d %d %d", &t, &u, &v);
    if (t == 0) {
      ds.merge(u, v);
    } else {
      printf("%d\n", ds.same(u, v));
    }
  }
  return 0;
}
