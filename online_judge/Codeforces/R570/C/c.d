void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int q;
  rd(q);
  bool enough(long k, long n, long a, long b, long m) {
    return a * m + b * (n - m) < k;
  }

  while (q--) {
    long k, n, a, b;
    rd(k, n, a, b);
    if (b * n >= k) {
      writeln(-1);
      continue;
    }
    long ok = 0, ng = n + 1;
    while (ng - ok > 1) {
      auto m = (ok + ng) / 2;
      if (enough(k, n, a, b, m)) {
        ok = m;
      } else {
        ng = m;
      }
    }
    writeln(ok);
  }

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
