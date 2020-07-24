void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  int len = 0;
  foreach (i; 0 .. n) {
    int x, y;
    rd(x, y);
    len = max(len, x + y);
  }
  writeln(len);
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
