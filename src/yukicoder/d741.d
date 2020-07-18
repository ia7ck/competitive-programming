void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  const int mod = 10 ^^ 9 + 7;
  auto memo = new int[][](n + 1, 10);
  afill(memo, -1);
  int f(int i, int prev) {
    if (i == n) {
      return 1;
    } else {
      if (memo[i][prev] >= 0) {
        return memo[i][prev];
      }
      int ret = 0;
      for (int digit = prev; digit <= 9; digit++) {
        (ret += f(i + 1, digit)) %= mod;
      }
      return memo[i][prev] = ret;
    }
  }

  writeln(f(0, 0));
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
