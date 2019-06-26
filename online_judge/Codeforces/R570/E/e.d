void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  long k;
  rd(n, k);
  auto s = readln.chomp.to!(char[]);

  auto dp = new long[][](n + 1, n + 1);
  dp[0][0] = 1;
  auto last_pos = new int[](26);
  fill(last_pos, -1);
  for (int i = 1; i <= n; i++) {
    foreach (j; 0 .. i) {
      dp[i][j] += dp[i - 1][j]; // not use s[i - 1]
      dp[i][j + 1] += dp[i - 1][j];
      if (last_pos[s[i - 1] - 'a'] >= 0) {
        dp[i][j + 1] -= dp[last_pos[s[i - 1] - 'a']][j];
      }
    }
    last_pos[s[i - 1] - 'a'] = i - 1;
  }
  long ans = 0;
  for (int j = n; j >= 0 && k > 0; j--) {
    if (k >= dp[n][j]) {
      ans += dp[n][j] * (n - j);
    } else {
      ans += k * (n - j);
    }
    k = max(0L, k - dp[n][j]);
  }
  if (k > 0) {
    writeln(-1);
  } else {
    writeln(ans);
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
