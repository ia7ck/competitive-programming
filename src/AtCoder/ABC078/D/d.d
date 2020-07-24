void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, z, w;
  rd(n, z, w);
  auto a = readln.split.to!(int[]);

  import std.math : abs;

  struct P {
    int i, t, p;
  }

  int[P] memo;
  int f(int i, int t, int pre) {
    if (i + 1 == n) {
      return abs(a[i] - pre);
    } else {
      auto key = P(i, t, pre);
      if (key in memo) {
        return memo[key];
      }
      int ret = t ? 2 * 10 ^^ 9 : 0;
      if (t) {
        for (int j = i; j + 1 < n; j++) {
          ret = min(ret, f(j + 1, t ^ 1, a[j]));
        }
        ret = min(ret, abs(a[n - 1] - pre));
      } else {
        for (int j = i; j + 1 < n; j++) {
          ret = max(ret, f(j + 1, t ^ 1, a[j]));
        }
        ret = max(ret, abs(a[n - 1] - pre));
      }
      return memo[key] = ret;
    }
  }

  writeln(f(0, 0, w));
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
