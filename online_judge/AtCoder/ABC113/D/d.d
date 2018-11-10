void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int h, w, k;
  rd(h, w, k);
  const int mod = 10 ^^ 9 + 7;

  auto dp = new int[](w);
  dp[0] = 1;
  foreach (_; 0 .. h) {
    auto nex = new int[](w);
    foreach (bit; 0 .. (1 << (w - 1))) {
      if (bit & (bit >> 1))
        continue;
      foreach (i; 0 .. w) {
        if (i - 1 >= 0 && bit & (1 << (i - 1))) {
          (nex[i - 1] += dp[i]) %= mod;
        } else if (i < w - 1 && bit & (1 << i)) {
          (nex[i + 1] += dp[i]) %= mod;
        } else {
          (nex[i] += dp[i]) %= mod;
        }
      }
    }
    dp.swap(nex);
  }

  writeln(dp[k - 1]);
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
