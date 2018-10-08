void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long n, m;
  rd(n, m);
  import std.math : sqrt;

  for (long k = m; k >= 1; k--) {
    if (m >= k * n && (m - k * n) % k == 0) {
      writeln(k);
      return;
    }
  }

  import std.exception : enforce;

  enforce(false);
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
