void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int q;
  rd(q);
  while (q--) {
    long n, a, b;
    rd(n, a, b);
    auto ans = n * a;
    if (n & 1) {
      ans = min(ans, n / 2 * b + a);
    } else {
      ans = min(ans, n / 2 * b);
    }
    writeln(ans);
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
