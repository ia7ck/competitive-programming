void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = new int[](n);
  foreach (i; 0 .. n)
    rd(a[i]);

  bool[int] map;
  foreach (e; a) {
    if (e in map)
      map.remove(e);
    else
      map[e] = true;
  }
  writeln(map.length);
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
