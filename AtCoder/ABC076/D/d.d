void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto t = readln.split.to!(int[]);
  auto v = readln.split.to!(int[]);

  t.each!((ref _t) => (_t *= 2));
  v.each!((ref _v) => (_v *= 2));
  auto T = reduce!"a+b"(t), V = reduce!(max)(v);
  auto dp = new int[][](T + 1, V + 1);
  for (int i = 0; i <= T; i++) {
    for (int j = 0; j <= V; j++) {
      dp[i][j] = -1;
    }
  }
  dp[0][0] = 0;
  int tm = 0;
  foreach (i; 0 .. n) {
    foreach (j; tm .. (tm + t[i])) {
      for (int k = 0; k <= v[i]; k++) {
        if (dp[j][k] >= 0) {
          chmax(dp[j + 1][k], dp[j][k] + k * 2);
          if (k + 1 <= v[i]) {
            chmax(dp[j + 1][k + 1], dp[j][k] + k * 2 + 1);
          }
          if (k - 1 >= 0) {
            chmax(dp[j + 1][k - 1], dp[j][k] + k * 2 - 1);
          }
        }
      }
    }
    tm += t[i];
  }
  writefln("%.18f", dp[T][0] / 8.0);
}

void chmax(T)(ref T l, T r) {
  if (l < r)
    l = r;
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
