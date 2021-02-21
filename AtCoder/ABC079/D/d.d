void main() {
  import std.stdio, std.string, std.conv, std.algorithm;
  import std.range : iota;

  int h, w;
  rd(h, w);
  const int n = 10;
  auto c = new int[][](n, n);
  iota(0, n).each!((i) => (c[i] = readln.split.to!(int[])));
  foreach (_; 0 .. n)
    foreach (i; 0 .. n)
      foreach (j; 0 .. n)
        c[i][j] = min(c[i][j], c[i][_] + c[_][j]);

  int cost = 0;
  foreach (_; 0 .. h) {
    auto a = readln.split.to!(int[]);
    cost += reduce!((res, val) => res + (val < 0 ? 0 : c[val][1]))(0, a);
  }
  writeln(cost);
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
