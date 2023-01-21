#include <iostream>
#include <vector>
#include <cassert>
#include "atcoder/math"

#define REP(i, n) for (int i = 0; i < (int)(n); i++)
using namespace std;

int main() {
    vector<long long> m = {4, 5, 7, 9, 11, 13, 17, 19, 23};
    vector<int> a;
    {
        int offset = 0;
        for (const auto &l: m) {
            vector<int> sub;
            REP(j, l) {
                sub.push_back(offset + j + 1);
            }
            REP(j, l) {
                a.push_back(sub[(j + 1) % l]);
            }

            offset += l;
        }
    }

    cout << a.size() << endl;
    REP(i, a.size()) {
        cout << a[i];
        if (i + 1 < a.size()) {
            cout << " ";
        } else {
            cout << "\n";
        }
    }

    vector<int> b(a.size());
    REP(i, b.size()) {
        cin >> b[i];
    }

    vector<long long> rot;
    {
        int offset = 0;
        for (const auto &l: m) {
            int pos = -1;
            REP(j, l) {
                if (b[offset + j] == offset + 1) {
                    pos = j;
                    break;
                }
            }
            assert(pos != -1);
            rot.push_back(pos == 0 ? 0 : l - pos);
            offset += l;
        }
    }

    auto [y, z] = atcoder::crt(rot, m);
    cerr << z << endl;
    assert(y <= 1000000000);
    cout << y << endl;

    return 0;
}
