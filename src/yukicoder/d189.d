void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto args = readln.split.to!(string[]);
  const long mod = 10 ^^ 9 + 9;
  long[][] solve(string n) {
    auto dp = new long[][][](n.length + 1, 2, n.length * 10);
    dp[0][0][0] = 1;
    foreach (i; 0 .. n.length) {
      foreach (l; 0 .. 2) {
        foreach (s; 0 .. (n.length * 10)) {
          auto d = l ? 9 : n[i] - '0';
          for (int digit = 0; digit <= d; digit++) {
            auto less = l || (digit < d);
            auto sum = s + digit;
            if (sum < n.length * 10) {
              (dp[i + 1][less][sum] += dp[i][l][s]) %= mod;
            }
          }
        }
      }
    }
    return dp[n.length];
  }

  auto dp1 = solve(args[0]), dp2 = solve(args[1]);
  long tot = 0;
  foreach (i; 0 .. 2) {
    foreach (j; 0 .. 2) {
      foreach (s; 1 .. min(dp1[i].length, dp2[j].length)) {
        (tot += dp1[i][s] * dp2[j][s] % mod) %= mod;
      }
    }
  }
  writeln(tot);
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
