void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int t;
  rd(t);
  while (t--) {
    long n, v, l, r;
    rd(n, v, l, r);
    auto num = (l - 1) / v;
    num += n / v - r / v;
    writeln(num);
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
