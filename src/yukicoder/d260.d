void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto args = readln.split.to!(string[]);
  const int MOD = 10 ^^ 9 + 7;
  int solve(string n) {
    auto dp = new int[][][][][](n.length + 1, 2, 2, 3, 8);
    dp[0][0][0][0][0] = 1;
    foreach (i; 0 .. n.length) {
      foreach (less; 0 .. 2) {
        foreach (ok; 0 .. 2) {
          foreach (mod3; 0 .. 3) {
            foreach (mod8; 0 .. 8) {
              auto digit = less ? 9 : n[i] - '0';
              for (int d = 0; d <= digit; d++) {
                auto l = less || (d < digit);
                auto o = ok || (d == 3);
                auto m3 = (mod3 * 10 + d) % 3;
                auto m8 = (mod8 * 10 + d) % 8;
                (dp[i + 1][l][o][m3][m8] += dp[i][less][ok][mod3][mod8]) %= MOD;
              }
            }
          }
        }
      }
    }
    int ret = 0;
    foreach (l; 0 .. 2) {
      foreach (o; 0 .. 2) {
        foreach (m3; 0 .. 3) {
          foreach (m8; 0 .. 8) {
            if ((o || m3 == 0) && (m8 > 0)) {
              (ret += dp[n.length][l][o][m3][m8]) %= MOD;
            }
          }
        }
      }
    }
    return ret;
  }

  bool yes(string n) {
    auto m8 = 0;
    foreach (ch; n) {
      (m8 *= 10) %= 8;
      (m8 += ch - '0') %= 8;
    }
    if (m8 == 0) {
      return false;
    } else if (n.count('3') > 0) {
      return true;
    } else {
      auto m3 = reduce!((res, ch) => (res + (ch - '0')) % 3)(0, n);
      return m3 == 0;
    }
  }

  auto res = (solve(args[1]) - solve(args[0]) + MOD) % MOD;
  writeln((res + yes(args[0])) % MOD);
}

void afill(Range, Type)(ref Range r, Type v) {
  static if (is(typeof(r) == Type[])) {
    foreach (ref e; r) {
      e = v;
    }
  } else {
    foreach (ref _r; r) {
      afill(_r, v);
    }
  }
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
