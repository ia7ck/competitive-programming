void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import core.bitop : popcnt;

  const mod = 10 ^^ 8 + 7;
  long powmod(long b, long e) {
    if (e == 0)
      return 1;
    else if (e == 1)
      return b;
    else if (e & 1)
      return b * powmod(b, e - 1) % mod;
    else
      return powmod(b * b % mod, e / 2);
  }

  int h, w, k;
  rd(h, w, k);
  struct P {
    int y, x, idx;
  }

  auto fac = new long[](h + w + 1), inv = new long[](h + w + 1);
  fac[0] = inv[0] = 1;
  foreach (i; 1 .. (h + w + 1)) {
    fac[i] = fac[i - 1] * i % mod;
    inv[i] = powmod(fac[i], mod - 2);
  }
  long cmb(int n, int r) {
    if (r < 0 || n < r)
      return 0;
    else
      return fac[n] * inv[r] % mod * inv[n - r] % mod;
  }

  auto abi = new P[](k);
  foreach (i; 0 .. k) {
    int y, x;
    rd(y, x);
    abi[i] = P(y, x, i);
  }
  abi.sort!((p, q) => (p.y == q.y ? p.x < q.x : p.y < q.y));
  auto dp = new long[](1 << k);
  fill(dp, 1);
  foreach (bits; 0 .. (1 << k)) {
    int cur_x = 0, cur_y = 0;
    foreach (p; abi) {
      if (bits & (1 << p.idx)) {
        int dy = p.y - cur_y, dx = p.x - cur_x;
        dp[bits] = dp[bits] * cmb(dy + dx, dy) % mod;
        cur_y = p.y;
        cur_x = p.x;
      }
    }
    dp[bits] = dp[bits] * cmb((h - cur_y) + (w - cur_x), h - cur_y) % mod;
  }
  foreach (i; 0 .. k) {
    foreach (bits; 0 .. (1 << k)) {
      if ((bits & (1 << i)) == 0) {
        dp[bits ^ (1 << i)] = (dp[bits ^ (1 << i)] + (mod - dp[bits])) % mod;
      }
    }
  }
  foreach (bits, x; dp) {
    if (popcnt(bits) & 1)
      writeln(mod - x);
    else
      writeln(x);
  }
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
