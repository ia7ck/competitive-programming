int next(int x) {
  x += 1;
  while (x % 10 == 0) {
    x /= 10;
  }
  return x;
}

void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);

  bool[int] seen;
  while ((n in seen) == null) {
    seen[n] = true;
    n = next(n);
  }
  writeln(seen.length);
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
