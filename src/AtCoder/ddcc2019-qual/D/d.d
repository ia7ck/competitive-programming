void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  const int n = 30;
  auto a = new long[](n + 1);
  long[] b, m;
  for (int i = 2; i <= n; i++) {
    rd(a[i]);
    b ~= a[i] % (i - 1);
    m ~= i - 1;
  }
  long[] res = [0, 1];
  for (int i = 0; i < b.length; i++) {
    res = crt(res[0], b[i], res[1], m[i]);
    if (res[0] < 0 || res[0] > 10L ^^ 12) {
      writeln("invalid");
      return;
    }
  }
  for (int i = 2; i <= n; i++) {
    if (digit_sum(res[0], i) != a[i]) {
      writeln("invalid");
      return;
    }
  }
  writeln(res[0]);
}

long digit_sum(long n, long b) {
  long ret = 0;
  while (n) {
    ret += n % b;
    n /= b;
  }
  return ret;
}

long[] ext_gcd(long a, long b) {
  if (b == 0) {
    return [1, 0, a];
  } else {
    auto xyg = ext_gcd(b, a % b);
    return [xyg[1], xyg[0] - a / b * xyg[1], xyg[2]];
  }
}

long[] crt(long a1, long a2, long m1, long m2) {
  auto xyg = ext_gcd(m1, m2);
  if ((a1 - a2) % xyg[2] > 0) {
    return [-1, 0];
  }
  auto ret = a1 + (a2 - a1) / xyg[2] % m2 * m1 * (xyg[0] % m2), lcm = m1 * m2 / xyg[2];
  return [(ret % lcm + lcm) % lcm, lcm];
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
