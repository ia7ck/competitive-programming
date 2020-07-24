void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long a;
  int k;
  rd(a, k);
  long _a = a;
  int[] n;
  while (_a) {
    n = (_a % 10) ~ n;
    _a /= 10;
  }

  long[] pow = [1];
  foreach (_; 0 .. 15) {
    pow ~= pow[$ - 1] * 10;
  }

  import core.bitop : popcnt;

  const long inf = 10L ^^ 18;
  auto memo = new long[][][][](n.length + 1, 2, 1 << 10, 2);
  auto memo2 = new long[][][][](n.length + 1, 2, 1 << 10, 2);
  foreach (i; 0 .. (n.length + 1))
    foreach (j; 0 .. 2)
      foreach (x; 0 .. (1 << 10))
        foreach (y; 0 .. 2)
          memo[i][j][x][y] = -inf, memo2[i][j][x][y] = -inf;

  long f(size_t i, bool less, int used, bool zero) { // a以下で最大
    if (i == n.length) {
      return 0;
    } else {
      if (memo[i][less][used][zero] > -inf)
        return memo[i][less][used][zero];
      long ret = -1;
      auto digit = less ? 9 : n[i];
      for (int d = 0; d <= digit; d++) {
        auto l = less || (d < digit);
        auto z = zero || (d > 0);
        auto u = !z ? 0 : (used | (1 << d));
        if (popcnt(u) <= k) {
          auto res = f(i + 1, l, u, z);
          if (res >= 0) { // res = -1 ならアレ
            ret = max(ret, d * pow[n.length - i - 1] + res);
          }
        }
      }
      return memo[i][less][used][zero] = ret;
    }
  }

  long f2(size_t i, bool greater, int used, bool zero) { // a以上で最小
    if (i == n.length) {
      return 0;
    } else {
      if (memo2[i][greater][used][zero] >= 0)
        return memo2[i][greater][used][zero];
      long ret = inf;
      auto digit = greater ? 0 : n[i];
      for (int d = digit; d <= 9; d++) {
        auto g = greater || (d > digit);
        auto z = zero || (d > 0);
        auto u = !z ? 0 : (used | (1 << d));
        if (popcnt(u) <= k) {
          ret = min(ret, d * pow[n.length - i - 1] + f2(i + 1, g, u, z));
        }
      }
      return memo2[i][greater][used][zero] = ret;
    }
  }

  writeln(min(a - f(0, 0, 0, 0), f2(0, 0, 0, 0) - a));
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
