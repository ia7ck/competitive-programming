void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m;
  rd(n, m);

  int[] a;
  for (int i = 2; i * i <= m; i++) {
    if (m % i == 0) {
      int cnt = 0;
      while (m % i == 0) {
        cnt++;
        m /= i;
      }
      a ~= cnt;
    }
  }
  if (m > 1) {
    a ~= 1;
  }

  const long mod = 10 ^^ 9 + 7;
  const size_t M = 10 ^^ 5;
  static long[M] fac, inv;
  { // init
    fac[0] = fac[1] = 1;
    foreach (i; 2 .. M)
      fac[i] = i * fac[i - 1] % mod;
    long _pow(long a, long x) {
      if (x == 0)
        return 1;
      else if (x == 1)
        return a;
      else if (x & 1)
        return a * _pow(a, x - 1) % mod;
      else
        return _pow(a * a % mod, x / 2);
    }

    foreach (i; 0 .. M)
      inv[i] = _pow(fac[i], mod - 2);
  }
  long cmb(long nn, long rr) {
    if (nn < rr)
      return 0;
    else
      return fac[nn] * inv[rr] % mod * inv[nn - rr] % mod;
  }

  long tot = 1;
  foreach (e; a) {
    (tot *= cmb(e + n - 1, e)) %= mod;
  }
  writeln(tot);
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
