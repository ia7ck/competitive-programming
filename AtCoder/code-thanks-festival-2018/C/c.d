void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(long[]);

  sort(a);
  reverse(a);
  long s = 0;
  foreach (i, v; a) {
    s += v * (n - i - 1);
    s -= v * i;
  }
  writeln(s);
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
