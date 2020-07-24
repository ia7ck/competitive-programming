void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  string a, b;
  rd(a, b);

  long solve(string n, bool flag) {
    auto dp = new long[][][](n.length + 1, 2, 2);
    dp[0][0][0] = 1;
    foreach (i; 0 .. n.length) {
      foreach (l; 0 .. 2) {
        foreach (h; 0 .. 2) {
          auto d = l ? 9 : n[i] - '0';
          for (int digit = 0; digit <= d; digit++) {
            auto less = l || (digit < d);
            auto has = h || (digit == 4 || digit == 9);
            dp[i + 1][less][has] += dp[i][l][h];
          }
        }
      }
    }
    long ret = dp[n.length][0][1] + dp[n.length][1][1];
    if (flag) {
      ret -= dp[n.length][0][1];
    }
    return ret;
  }

  writeln(solve(b, false) - solve(a, true));
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
