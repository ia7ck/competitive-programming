void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int t, a, b, c, d;
  rd(t, a, b, c, d);

  int best = 0;
  if (a <= t) {
    best = max(best, b);
  }
  if (c <= t) {
    best = max(best, d);
  }
  if (a + c <= t) {
    best = max(best, b + d);
  }
  writeln(best);

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
