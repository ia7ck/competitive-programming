#include <iostream>
#include <vector>
#include <algorithm>
#include <cassert>
#include <set>
#include <limits>

int64_t expr(int &pos);
int64_t term1(int &pos);
int64_t term2(int &pos);
int64_t term3(int &pos);
int64_t number(int &pos);

using namespace std;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)
string s;
set<char> op1, op2, op3; // 優先度 op1 < op2 < op3

void debug(int pos) {
  cerr << "read string: " << s.substr(0, pos) << endl;
  cerr << "s[pos] = '" << s[pos] << "'" << endl;
}

int64_t expr(int &pos) {
  // debug(pos);
  auto res = term1(pos);
  // debug(pos);
  while (pos < s.size() and op1.count(s[pos])) {
    auto op = s[pos++];
    if (op == '+') {
      res += term1(pos);
    } else if(op == '-') {
      res -= term1(pos);
    } else {
      res *= term1(pos);
    }
  }
  return res;
}

int64_t term1(int &pos) {
  auto res = term2(pos);
  while (pos < s.size() and op2.count(s[pos])) {
    auto op = s[pos++];
    if (op == '+') {
      res += term2(pos);
    } else if(op == '-') {
      res -= term2(pos);
    } else {
      res *= term2(pos);
    }
  }
  return res;
}

int64_t term2(int &pos) {
  auto res = term3(pos);
  while (pos < s.size() and op3.count(s[pos])) {
    auto op = s[pos++];
    if (op == '+') {
      res += term3(pos);
    } else if(op == '-') {
      res -= term3(pos);
    } else {
      res *= term3(pos);
    }
  }
  return res;
}

bool is_digit(char ch) {
  return '0' <= ch and ch <= '9';
}

int64_t term3(int &pos) {
  if (is_digit(s[pos])) {
    return number(pos);
  }
  assert(s[pos++] == '(');
  auto res = expr(pos);
  assert(s[pos++] == ')');
  return res;
}

int64_t number(int &pos) {
  int64_t res = 0;
  while (pos < s.size() and is_digit(s[pos])) {
    res = res * 10 + (s[pos++] - '0');
  }
  return res;
}

int main() {
  cin >> s;
  vector<char> ops = {'+', '-', '*'};
  int64_t ans = numeric_limits<int64_t>::min();
  rep(b1, 1 << 3) {
    if (b1 == 0) continue;
    rep(b2, 1 << 3) {
      if (b1 != (1 << 3) - 1 and b2 == 0) continue;
      if (b1 & b2) continue;
      rep(b3, 1 << 3) {
        if (b1 & b3) continue;
        if (b2 & b3) continue;
        if ((b1 | b2 | b3) != (1 << 3) - 1) continue;
        op1.clear();
        op2.clear();
        op3.clear();
        rep(i, 3) {
          if (b1 >> i & 1) op1.insert(ops[i]);
          if (b2 >> i & 1) op2.insert(ops[i]);
          if (b3 >> i & 1) op3.insert(ops[i]);
        }
        // for (auto it: op1) cerr << it << ","; cerr << "|";
        // for (auto it: op2) cerr << it << ","; cerr << "|";
        // for (auto it: op3) cerr << it << ",";
        // cerr << endl;
        // <expr> ::= <term1> [('+'|'-'|'*') <term1>]*
        // <term1> ::= <term2> [('+'|'-'|'*') <term2>]*
        // <term2> ::= <term3> [('+'|'-'|'*') <term3>]*
        // <term3> ::= <number> | '(' <expr> ')'
        // <number> ::= 123 など
        int pos = 0;
        auto res = expr(pos);
        // cerr << res << endl;
        ans = max(ans, res);
      }
    }
  }
  cout << ans << endl;
  return 0;
}
