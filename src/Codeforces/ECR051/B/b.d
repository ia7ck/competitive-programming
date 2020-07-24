void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long l, r;
  rd(l, r);
  writeln("YES");
  for (auto i = l; i + 1 <= r; i += 2) {
    writeln(i, " ", i + 1);
  }
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
