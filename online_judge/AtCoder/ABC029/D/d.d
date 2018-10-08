void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  string n;
  rd(n);
  auto memo = new long[][][](n.length + 1, 2, 11);
  afill(memo, -1L);
  long f(size_t i, bool less, int cnt) {
    if (i == n.length) {
      return cnt;
    } else {
      if (memo[i][less][cnt] >= 0) {
        return memo[i][less][cnt];
      }
      long ret = 0;
      auto digit = less ? 9 : n[i] - '0';
      for (int d = 0; d <= digit; d++) {
        auto l = less || (d < digit);
        auto c = cnt + (d == 1);
        ret += f(i + 1, l, c);
      }
      return memo[i][less][cnt] = ret;
    }
  }

  writeln(f(0, 0, 0));
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
