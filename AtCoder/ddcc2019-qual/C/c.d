void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long n;
  rd(n);
  const long mod = 10 ^^ 9 + 7;
  long ans = 0;
  for (auto i = 1L; i <= n; i++) {
    auto p = 1L, q = 1L, r = 1L;
    foreach (_; 0 .. 10) {
      (p *= i) %= mod;
      (q *= (i - 1)) %= mod;
      (r *= (n / i)) %= mod;
    }
    (ans += (p - q + mod) % mod * r % mod) %= mod;
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
