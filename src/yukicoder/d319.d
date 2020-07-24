void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long a, b;
  rd(a, b);

  long f(long _n) {
    int[] n;
    do {
      n = _n % 10 ~ n;
      _n /= 10;
    }
    while (_n);
    auto memog = new long[][][][](n.length + 1, 2, 10, n.length + 1);
    afill(memog, -1L);
    long g(size_t i, bool less, int prev, int count12) {
      if (i == n.length) {
        return count12;
      } else {
        if (memog[i][less][prev][count12] >= 0) {
          return memog[i][less][prev][count12];
        }
        long ret = 0;
        auto digit = less ? 9 : n[i];
        for (int d = 0; d <= digit; d++) {
          auto l = less || (d < digit);
          auto p = d;
          auto c12 = count12 + (i > 0 && prev == 1 && d == 2);
          ret += g(i + 1, l, p, c12);
        }
        return memog[i][less][prev][count12] = ret;
      }
    }

    auto memoh = new long[][][][](n.length + 1, 2, 10, 10);
    afill(memoh, -1L);
    long h(size_t i, bool less, int first, int last) {
      if (i == n.length) {
        return first == 2 && last == 2;
      } else {
        if (memoh[i][less][first][last] >= 0) {
          return memoh[i][less][first][last];
        }
        long ret = 0;
        auto digit = less ? 9 : n[i];
        for (int d = 0; d <= digit; d++) {
          auto l = less || (d < digit);
          auto fi = first == 0 && d > 0 ? d : first;
          auto la = d;
          ret += h(i + 1, l, fi, la);
        }
        return memoh[i][less][first][last] = ret;
      }
    }

    return g(0, 0, 0, 0) + h(0, 0, 0, 0);
  }

  auto res = f(b) - f(a - 1);
  {
    auto last = a % 10, first = -1;
    while (a) {
      first = a % 10;
      a /= 10;
    }
    if (first == 2 && last == 2) {
      res--;
    }
  }
  writeln(res);
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

void rd(T...)(ref T x) {
  import std.stdio : readln;
  import std.string : split;
  import std.conv : to;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
