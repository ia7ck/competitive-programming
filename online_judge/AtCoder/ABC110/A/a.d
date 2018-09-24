void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int a, b, c;
  rd(a, b, c);

  writeln(max(a * 10 + b + c, b * 10 + c + a, c * 10 + a + b));
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
