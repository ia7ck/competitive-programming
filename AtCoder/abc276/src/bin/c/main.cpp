#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
#define REP(i, n) for (int i = 0; i < (int)n; i++)

int main() {
    int n;
    cin >> n;
    vector<int> p(n);
    REP(i, n) {
        cin >> p[i];
    }

    prev_permutation(p.begin(), p.end());
    REP(i, n) {
        cout << p[i];
        if (i + 1 < n) {
            cout << ' ';
        } else {
            cout << '\n';
        }
    }
}
