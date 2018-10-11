void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  long n, m;
  rd(n, m);

  auto mn = max(0, n - m * 2);
  long mx = 0;
  for (auto k = 0L; k <= n; k++) {
    if (k * (k - 1) / 2 >= m) {
      mx = max(mx, n - k);
    }
  }
  writeln(mn, " ", mx);
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
