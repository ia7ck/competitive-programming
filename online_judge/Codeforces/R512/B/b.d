void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, d;
  rd(n, d);
  struct P {
    int x, y;
  }

  P[] vec = [P(d, -d), P(n - d, n - d), P(-d, d), P(d - n, d - n)];
  P[] ps = [P(0, d), P(d, 0), P(n, n - d), P(n - d, n)];
  int m;
  rd(m);
  while (m--) {
    int x, y;
    rd(x, y);
    int[] o;
    foreach (i, p; ps) {
      auto u = P(x - p.x, y - p.y);
      o ~= vec[i].x * u.y - vec[i].y * u.x;
    }
    auto inside = reduce!((r, e) => (r && e >= 0))(true, o);
    if (inside) {
      writeln("YES");
    } else {
      writeln("NO");
    }
  }
}

void rd(T...)(ref T x) {
  import std.stdio, std.string, std.conv;

  auto l = readln.split;
  assert(l.length == x.length);
  foreach (i, ref e; x)
    e = l[i].to!(typeof(e));
}
