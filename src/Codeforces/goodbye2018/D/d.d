void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  const long mod = 998244353;

  long powmod(long b, long e) {
    if (e == 0)
      return 1;
    else if (e == 1)
      return b % mod;
    else if (e & 1)
      return b * powmod(b, e - 1) % mod;
    else
      return powmod(b * b % mod, e / 2);
  }

  auto fac = new long[](n + 1), inv = new long[](n + 1);
  fac[0] = inv[0] = 1;
  for (int i = 1; i <= n; i++) {
    fac[i] = fac[i - 1] * i % mod;
    inv[i] = powmod(fac[i], mod - 2);
  }
  long ans = n * fac[n] % mod;
  foreach (i; 1 .. n) {
    ans -= fac[n] * inv[i] % mod;
    if (ans < 0) {
      ans += mod;
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
