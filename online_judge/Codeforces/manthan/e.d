void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int[] n;
  auto memo = new long[][][][](11, 64, 1 << 10, 2);
  afill(memo, -1L);
  long f(const int base, int i, bool less, int used, bool zero) {
    if (i < 0) {
      return used == 0 && !zero;
    } else {
      if (less && memo[base][i][used][zero] >= 0) {
        return memo[base][i][used][zero];
      }
      long ret = 0;
      auto digit = less ? base - 1 : n[i];
      for (int d = 0; d <= digit; d++) {
        auto l = less || (d < digit);
        auto z = zero && (d == 0);
        auto u = used ^ (z ? 0 : (1 << d));
        ret += f(base, i - 1, l, u, z);
      }
      if (less) {
        return memo[base][i][used][zero] = ret;
      } else {
        return ret;
      }
    }
  }

  int q;
  rd(q);
  while (q--) {
    int b;
    long l, r;
    rd(b, l, r);
    long res = 0;
    n = digit(r, b).dup;
    res += f(b, (n.length - 1).to!(int), 0, 0, true);
    n = digit(l - 1, b).dup;
    res -= f(b, (n.length - 1).to!(int), 0, 0, true);
    writeln(res);
  }
}

int[] digit(long n, int base) {
  int[] ret;
  do {
    ret ~= n % base;
    n /= base;
  }
  while (n);
  return ret;
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

void afill(Range, Type)(Range r, Type value) {
  static if (is(typeof(r) == Type[])) {
    foreach (ref elem; r)
      elem = value;
  } else {
    foreach (ref arr; r)
      afill(arr, value);
  }
}
