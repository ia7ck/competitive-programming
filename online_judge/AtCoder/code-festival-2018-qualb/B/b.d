void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, x;
  rd(n, x);
  auto a = new int[](n), b = new int[](n);
  foreach (i; 0 .. n)
    rd(a[i], b[i]);

  int tot = 0;
  foreach (i; 0 .. n)
    tot += a[i] * b[i];
  writeln(tot + x * reduce!(max)(b));
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
