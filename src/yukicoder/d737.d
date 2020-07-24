void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long n;
  rd(n);
  const long mod = 10 ^^ 9 + 7;

  int get(long nn) {
    int ret = 0;
    foreach (i; 0 .. 62) {
      if (nn & (1L << i))
        ret = i;
    }
    return ret + 1;
  }

  int len = get(n);
  auto dp = new long[][][](len + 1, len + 1, 2);
  auto o = new long[][][](len + 1, len + 1, 2);
  dp[0][0][0] = 1; // ??
  foreach (i; 0 .. len) {
    for (int j = 0; j <= len; j++) {
      foreach (f; 0 .. 2) {
        foreach (b; 0 .. 2) {
          auto x = (n >> (len - i - 1)) & 1;
          if (f == 0 && x == 0 && b == 1) {
            continue;
          }
          auto nj = j + b, nf = f | (b < x);
          if (nj <= len) {
            (dp[i + 1][nj][nf] += dp[i][j][f]) %= mod;
            (o[i + 1][nj][nf] += o[i][j][f] * 2) %= mod;
            if (b) {
              (o[i + 1][nj][nf] += dp[i][j][f]) %= mod;
            }
          }
        }
      }
    }
  }
  long s = 0;
  for (int j = 1; j <= len; j++) {
    foreach (f; 0 .. 2) {
      (s += o[len][j][f] * j) %= mod;
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
