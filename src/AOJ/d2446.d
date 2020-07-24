void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.numeric, std.math;

  int n;
  long m;
  rd(n, m);
  auto a = readln.split.to!(long[]);
  auto p = readln.split.to!(int[]);

  auto dp = new long[](1 << n);
  foreach (bit; 1 .. (1 << n)) {
    auto mul = 1L;
    foreach (i; 0 .. n) {
      if (bit & (1 << i)) {
        auto g = gcd(mul, a[i]), b = mul / g;
        if (1.0 * b * a[i] > m) {
          mul = m + 1;
          break;
        }
        mul = mul / g * a[i];
      }
    }
    dp[bit] = m / mul;
  }
  foreach (i; 0 .. n) {
    foreach (bit; 0 .. (1 << n)) {
      if ((bit & (1 << i)) == 0) {
        dp[bit ^ (1 << i)] -= dp[bit];
      }
    }
  }
  auto ans = 0.0;
  foreach (bit; 0 .. (1 << n)) {
    auto q = 1.0;
    foreach (i; 0 .. n) {
      if (bit & (1 << i)) {
        q *= p[i] / 100.0;
      } else {
        q *= (100 - p[i]) / 100.0;
      }
    }
    ans += (dp[bit].abs) * q;
  }
  writefln("%.18f", ans);

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
