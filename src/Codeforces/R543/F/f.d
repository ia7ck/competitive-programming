int[] make_z(in char[] s) {
  import std.algorithm : max;
  import std.conv : to;

  int n = s.length.to!(int);
  auto z = new int[](n);
  for (int i = 1, p = 0; i < n; i++) {
    if (i + z[i - p] < p + z[p]) {
      z[i] = z[i - p];
    } else {
      auto j = max(0, p + z[p] - i);
      while (i + j < n && s[j] == s[i + j]) {
        j++;
      }
      z[i] = j;
      p = i;
    }
  }
  z[0] = n;
  return z;
}

void chmin(ref int l, int r) {
  if (l > r) {
    l = r;
  }
}

void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, a, b;
  rd(n, a, b);
  auto s = readln.chomp.to!(char[]);

  auto z = new int[][](n, n);
  foreach (i; 0 .. n) {
    z[i][i .. $] = make_z(s[i .. $]);
  }
  auto dp = new int[](n + 1);
  fill(dp, 5000 * 5000);
  dp[0] = 0;
  foreach (i; 0 .. n) {
    chmin(dp[i + 1], dp[i] + a);
    foreach (j; 0 .. i) {
      if (z[j][i] > 0) {
        chmin(dp[i + min(z[j][i], i - j)], dp[i] + b);
      }
    }
  }
  writeln(dp[n]);
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
