void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long n, m;
  rd(n, m);
  long ans = 1;
  for (long k = 1; k * k <= m; k++) {
    if (m % k == 0) {
      if (n * k <= m) {
        ans = max(ans, k);
      }
      if (n * (m / k) <= m) {
        ans = max(ans, m / k);
      }
    }
  }
  writeln(ans);
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
