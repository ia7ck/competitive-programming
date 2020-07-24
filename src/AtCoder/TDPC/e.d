void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int m;
  string n;
  rd(m);
  rd(n);
  const int MOD = 10 ^^ 9 + 7;
  auto memo = new int[][][](n.length + 1, 2, m);
  afill(memo, -1);
  int f(size_t i, bool less, int sum) {
    if (i == n.length) {
      return sum == 0;
    } else {
      if (memo[i][less][sum] >= 0) {
        return memo[i][less][sum];
      }
      int ret = 0;
      auto digit = less ? 9 : n[i] - '0';
      for (int d = 0; d <= digit; d++) {
        auto l = less || (d < digit);
        auto s = (sum + d) % m;
        (ret += f(i + 1, l, s)) %= MOD;
      }
      return memo[i][less][sum] = ret;
    }
  }

  writeln(f(0, 0, 0) - 1);
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
