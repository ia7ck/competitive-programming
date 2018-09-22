void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  long k;
  rd(n, k);
  auto a = readln.split.to!(long[]);

  const long mod = 10 ^^ 9 + 7;
  auto cnt = new int[](n);
  foreach (i, _e; a) {
    auto e = _e;
    while (e > 0)
      e /= 2, cnt[i]++;
  }

  const int M = min(10000, k.to!(int));
  auto dp = new int[][](M + 1, 2);
  dp[0][0] = 1;
  foreach (i; 0 .. n) {
    auto nex = new int[][](M + 1, 2);
    for (int j = 0; j <= M; j++) {
      foreach (l; 0 .. cnt[i]) {
        if (j + l <= M) {
          (nex[j + l][0] += dp[j][0]) %= mod;
          (nex[j + l][1] += dp[j][1]) %= mod;
        }
      }
      if (j + cnt[i] <= M) {
        (nex[j + cnt[i]][1] += dp[j][1]) %= mod;
        (nex[j + cnt[i]][1] += dp[j][0]) %= mod;
      }
    }
    dp.swap(nex);
  }
  int s = 0;
  for (int i = 0; i <= M; i++)
    (s += dp[i][1]) %= mod;
  writeln((dp[M][0] + s) % mod);
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
