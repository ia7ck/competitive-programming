void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long a, b, x;
  rd(a, b, x);

  long mod = 1_000_000_000 + 7;
  auto k = (x - a + 1 + (a - b - 1)) / (a - b);
  if (k < 0) {
    k = 0;
  }
  auto ans = x % mod + (k % mod) * (b % mod);
  writeln(ans % mod);
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
