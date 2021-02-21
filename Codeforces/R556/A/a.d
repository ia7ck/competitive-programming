void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m, r;
  rd(n, m, r);
  auto a = readln.split.to!(int[]);
  auto b = readln.split.to!(int[]);

  auto dp = new int[][](n + 1, r + 1);
  for (int i = 0; i <= n; i++) {
    fill(dp[i], -1);
  }
  dp[0][0] = 0;
  foreach (i; 0 .. n) {
    for (int j = 0; j <= r; j++) {
      if (dp[i][j] >= 0) {
        dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
        for (int k = 1; j + a[i] * k <= r; k++) {
          dp[i + 1][j + a[i] * k] = max(dp[i + 1][j + a[i] * k], dp[i][j] + k);
        }
      }
    }
  }
  int ans = r, bmax = reduce!(max)(b);
  for (int j = 0; j <= r; j++) {
    auto cnt = dp[n][j];
    if (cnt >= 0) {
      ans = max(ans, r - j + cnt * bmax);
    }
  }
  writeln(ans);
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
