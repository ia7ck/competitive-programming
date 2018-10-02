void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  int s = 0;
  foreach (_; 0 .. n) {
    int l, r;
    rd(l, r);
    s += (r - l + 1);
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
