void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  auto s = readln.chomp.to!(char[]);
  auto n = s.length;
  auto a = new long[](n + 1), c = new long[](n + 1), q = new long[](n + 1);
  foreach (i; 0 .. n) {
    a[i + 1] += a[i];
    c[i + 1] += c[i];
    q[i + 1] += q[i];
    if (s[i] == 'A')
      a[i + 1]++;
    if (s[i] == 'C')
      c[i + 1]++;
    if (s[i] == '?')
      q[i + 1]++;
  }
  long mod = 10 ^^ 9 + 7;
  long powmod(long b, long ex) {
    if (ex == 0)
      return 1L;
    if (ex & 1)
      return b * powmod(b, ex - 1) % mod;
    else
      return powmod(b * b % mod, ex / 2);
  }

  // (A, C), (?, C), (C, ?), (?, ?)
  long tot = 0;
  foreach (i; 0 .. n) {
    if (s[i] == 'B' || s[i] == '?') {
      if (i > 0 && i + 1 < n) {
        (tot += a[i] * (c[n] - c[i + 1]) % mod * powmod(3, q[i] + (q[n] - q[i + 1]))) %= mod;
        if (q[i] > 0) {
          (tot += q[i] * (c[n] - c[i + 1]) % mod * powmod(3, q[i] + (q[n] - q[i + 1]) - 1)) %= mod;
        }
        if (q[n] - q[i + 1] > 0) {
          (tot += a[i] * (q[n] - q[i + 1]) % mod * powmod(3, q[i] + (q[n] - q[i + 1]) - 1)) %= mod;
        }
        if (q[i] > 0 && (q[n] - q[i + 1]) > 0) {
          (tot += q[i] * (q[n] - q[i + 1]) % mod * powmod(3, q[i] + (q[n] - q[i + 1]) - 2)) %= mod;
        }
      }
    }
  }
  writeln(tot);
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
