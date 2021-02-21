void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long a, b, c, d;
  rd(a, b, c, d);

  if (d == 10) {
    writeln("Impossible");
    return;
  }
  int[] v;
  foreach (_; 0 .. b)
    v ~= 50;
  foreach (_; 0 .. a)
    v ~= 100;
  long s = 0, p = 1;
  foreach (i; 0 .. (a + b)) {
    if (i > 0 && i % 100 == 0) {
      p *= 2;
    }
    s += v[i] * p;
  }
  writeln("Possible");
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
