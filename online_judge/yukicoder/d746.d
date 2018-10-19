void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int p;
  rd(p);
  /*
  char[] n;
  n ~= "1";
  foreach (_; 0 .. p)
    n ~= "0";
  auto memo = new int[][][](n.length + 1, 2, 7);
  afill(memo, -1);
  int f(size_t i, bool less, int sum) {
    if (i == n.length) {
      return sum % 7 == 0;
    } else {
      if (memo[i][less][sum] >= 0) {
        return memo[i][less][sum];
      }
      auto digit = less ? 9 : n[i] - '0';
      int ret = 0;
      for (int d = 0; d <= digit; d++) {
        auto l = less || d < digit;
        auto s = (sum * 10 + d) % 7;
        ret += f(i + 1, l, s);
      }
      return memo[i][less][sum] = ret;
    }
  }

  writeln(f(0, 0, 0) - 1);
  */

  if (p == 0) {
    writeln(0);
    return;
  }
  auto s = "142857";
  char[] ans;
  ans ~= "0.";
  foreach (i; 0 .. p) {
    ans ~= s[i % s.length];
  }
  writeln(ans);
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
