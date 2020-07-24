void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int h, w;
  rd(h, w);
  int a, b;
  rd(a, b);

  auto p = h / a, q = w / b;
  writeln((h - p * a) * w + h * (w - q * b) - ((h - p * a) * (w - q * b)));
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
