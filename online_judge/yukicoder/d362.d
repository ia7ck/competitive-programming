void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.array;

  int t;
  rd(t);
  while (t--) {
    long k;
    rd(k);
    long kadocount(long _n) {
      int[] n;
      do {
        n = _n % 10 ~ n;
        _n /= 10;
      }
      while (_n);
      auto memo = new long[][][][](20, 2, 100, 4);
      afill(memo, -1L);
      long f(size_t i, bool less, int last2, int trailing) {
        if (i == n.length) {
          return trailing >= 3;
        } else {
          if (memo[i][less][last2][trailing] >= 0) {
            return memo[i][less][last2][trailing];
          }
          long ret = 0;
          auto digit = less ? 9 : n[i];
          for (int d = 0; d <= digit; d++) {
            auto l = less || (d < digit);
            auto t = min(3, trailing + (trailing > 0 || d > 0));
            if (i < 2 || t < 3 || kadoma2(last2 * 10 + d)) {
              ret += f(i + 1, l, (last2 * 10 + d) % 100, t);
            }
          }
          return memo[i][less][last2][trailing] = ret;
        }
      }

      return f(0, 0, 0, 0);
    }

    bool enough(long n) {
      auto kk = kadocount(n);
      return kk >= k;
    }

    long ng = 100, ok = 10L ^^ 18;
    while (ok - ng > 1) {
      auto m = (ng + ok) / 2;
      (enough(m) ? ok : ng) = m;
    }
    writeln(ok);
  }
}

bool kadoma2(int n) {
  int x = n / 100, y = n / 10 % 10, z = n % 10;
  return (y - x) * (y - z) > 0 && x != z;
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
