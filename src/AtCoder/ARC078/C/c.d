void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(long[]);

  foreach (i; 1 .. n)
    a[i] = a[i] + a[i - 1];
  long mn = 10L ^^ 18;
  import std.math;

  foreach (i; 0 .. (n - 1)) { // [0, i], (i, n)
    mn = min(mn, abs(a[i] - (a[n - 1] - a[i])));
  }
  writeln(mn);
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
