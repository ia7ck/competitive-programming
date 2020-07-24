void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto s = new char[][](2, n);
  foreach (i; 0 .. 2)
    s[i] = readln.chomp.to!(char[]);

  const long mod = 10 ^^ 9 + 7;
  auto dp = new long[][](2, n);
  if (s[0][0] == s[1][0])
    dp[0][0] += 3;
  else
    dp[1][1] += 6;
  foreach (i; 1 .. n) {
    if (s[0][i] == s[1][i]) {
      dp[0][i] = (dp[0][i - 1] * 2 + dp[1][i - 1]) % mod;
    } else if (i >= 2) {
      dp[1][i] = (dp[0][i - 2] * 2 + dp[1][i - 2] * 3) % mod;
    }
  }
  writeln((dp[0][n - 1] + dp[1][n - 1]) % mod);
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
