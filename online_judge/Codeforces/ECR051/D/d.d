void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, k;
  rd(n, k);

  auto dp = new int[][](4, k + 10);
  const int mod = 998244353;

  void add(ref int x, int y) {
    x += y;
    if (x >= mod)
      x -= mod;
  }

  dp[0][1] = dp[3][1] = 1;
  dp[1][2] = dp[2][2] = 1;
  for (int i = 1; i < n; i++) {
    auto nex = new int[][](4, k + 10);
    foreach (s; 0 .. (1 << 2)) {
      for (int j = 1; j <= k; j++) {
        foreach (t; 0 .. (1 << 2)) {
          if (s == t) {
            add(nex[t][j], dp[s][j]);
          } else if ((s ^ t) == ((1 << 2) - 1)) {
            if (s == 0 || s == 3) {
              if (j + 1 <= k) {
                add(nex[t][j + 1], dp[s][j]);
              }
            } else {
              if (j + 2 <= k) {
                add(nex[t][j + 2], dp[s][j]);
              }
            }
          } else if ((s == 0 || s == 3) && (t == 1 || t == 2)) {
            if (j + 1 <= k) {
              add(nex[t][j + 1], dp[s][j]);
            }
          } else {
            add(nex[t][j], dp[s][j]);
          }
        }
      }
    }
    dp.swap(nex);
  }
  int tot = 0;
  foreach (i; 0 .. (1 << 2))
    (tot += dp[i][k]) %= mod;
  writeln(tot);
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
