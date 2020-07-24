void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int t;
  rd(t);
  while (t--) {
    auto args = readln.split.to!(long[]);
    long solve(long nn) {
      auto n = digit(nn);
      auto dp = new long[][][](n.length + 1, 2, 20);
      dp[0][0][0] = 1;
      foreach (i; 0 .. n.length) {
        foreach (l; 0 .. 2) {
          foreach (c; 0 .. 19) {
            auto d = l ? 9 : n[i];
            for (int digit = 0; digit <= d; digit++) {
              auto less = l || (digit < d);
              auto count = c + (digit > 0);
              dp[i + 1][less][count] += dp[i][l][c];
            }
          }
        }
      }
      long ret = 0;
      foreach (l; 0 .. 2) {
        for (int c = 0; c <= 3; c++) {
          ret += dp[n.length][l][c];
        }
      }
      return ret;
    }

    writeln(solve(args[1]) - solve(args[0] - 1));
  }
}

int[] digit(long x) {
  int[] ret;
  do {
    ret = x % 10 ~ ret;
    x /= 10;
  }
  while (x);
  return ret;
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
