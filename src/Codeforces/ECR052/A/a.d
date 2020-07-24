void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int t;
  rd(t);
  while (t--) {
    long s, a, b, c;
    rd(s, a, b, c);
    auto n = s / c;
    writeln(n + n / a * b);
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
