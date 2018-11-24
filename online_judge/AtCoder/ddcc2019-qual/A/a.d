void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  writeln(4 ^^ n);
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
