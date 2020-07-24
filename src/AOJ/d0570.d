void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  string a, b;
  int m;
  rd(a);
  rd(b);
  rd(m);
  const int MOD = 10 ^^ 4;

  int solve(string n) {
    auto memo = new short[][][][][][](n.length + 1, 2, 4, 10, 3, m);
    afill(memo, (-1).to!(short));
    int f(size_t i, bool less, int foo, int pre, int sgn, int modm, int k) { // sgn 1=>up, 2=>down
      if (i == n.length) {
        return modm == 0;
      } else {
        if (memo[i][less][foo][pre][sgn][modm] >= 0) {
          return memo[i][less][foo][pre][sgn][modm];
        }
        short ret = 0;
        auto digit = less ? 9 : n[i] - '0';
        for (int d = 0; d <= digit; d++) {
          auto l = less || (d < digit);
          auto bar = min(3, foo + (foo == 0 ? (d > 0) : 1));
          auto p = d;
          auto s = d - pre;
          if (s > 0) {
            s = 1;
          } else if (s < 0) {
            s = 2;
          }
          if (bar < 2 || (bar == 2 && s != 0) || ((sgn == 1 && s == 2) || (sgn == 2 && s == 1))) {
            (ret += f(i + 1, l, bar, p, s, (modm * 10 + d) % m, k * 10 + d)) %= MOD;
          }
        }
        return memo[i][less][foo][pre][sgn][modm] = ret;
      }
    }

    return f(0, 0, 0, 0, 0, 0, 0);
  }

  bool zigzag(string n) {
    if (n.length == 1) {
      return true;
    } else {
      for (int i = 1, sgn = 0; i < n.length; i++) {
        if (i == 1) {
          sgn = n[i - 1] - n[i];
        } else {
          auto s = n[i - 1] - n[i];
          if (sgn * s >= 0) {
            return false;
          } else {
            sgn = s;
          }
        }
      }
    }
    return true;
  }

  auto res = (solve(b) - solve(a) + MOD) % MOD;
  if (zigzag(a)) {
    res++;
  }
  writeln(res % MOD);
}

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}

void afill(Range, Type)(Range r, Type value) {
  static if (is(typeof(r) == Type[])) {
    foreach (ref elem; r)
      elem = value;
  } else {
    foreach (ref arr; r)
      afill(arr, value);
  }
}
