void main() {
  import std.algorithm;

  const long mod = 998244353;
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

  foreach (i; 0 .. M)
    assert(fac[i] * inv[i] == 1);
  assert(cmb(4, 0) == 1);
  assert(cmb(4, 2) == 6);
  assert(cmb(4, 4) == 1);
  assert(cmb(4, 5) == 0);
}

/*
  https://beta.atcoder.jp/contests/agc025/submissions/2611132
  https://beta.atcoder.jp/contests/abc110/submissions/3260878

  todo: https://beta.atcoder.jp/contests/abc034/tasks/abc034_c
*/
