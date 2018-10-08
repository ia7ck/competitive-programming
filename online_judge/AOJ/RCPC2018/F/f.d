void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto n = readln.chomp.to!(char[]);

  auto dp = new long[][][][](n.length + 1, 2, 1000, 2);
  dp[0][0][0][0] = 1;
  foreach (i; 0 .. (n.length)) {
    foreach (l; 0 .. 2) {
      foreach (k; 0 .. 1000) {
        foreach (ok; 0 .. 2) {
          auto x = k / 100, y = (k / 10) % 10, z = k % 10;
          auto d = l ? 9 : (n[i] - '0');
          for (int digit = 0; digit <= d; digit++) {
            auto less = l || (digit < d);
            auto okok = ok || (x == 5 && y == 1 && digit == 3);
            dp[i + 1][less][y * 100 + z * 10 + digit][okok] += dp[i][l][k][ok];
          }
        }
      }
    }
  }
  long s = 0;
  foreach (j; 0 .. 2) {
    foreach (k; 0 .. 1000) {
      s += dp[n.length][j][k][1];
    }
  }
  writeln(s);
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
