void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  struct E {
    int to;
    long cost;
  }

  auto g = new E[][](n);
  foreach (_; 0 .. n - 1) {
    int a, b;
    long c;
    rd(a, b, c);
    g[a - 1] ~= E(b - 1, c);
    g[b - 1] ~= E(a - 1, c);
  }
  int q, k;
  rd(q, k);
  auto dist = new long[](n);
  fill(dist, 10L ^^ 18);
  void dfs(int i, long d) {
    if (dist[i] < 10L ^^ 18)
      return;
    dist[i] = d;
    foreach (e; g[i]) {
      dfs(e.to, d + e.cost);
    }
  }

  dfs(k - 1, 0);
  while (q--) {
    int x, y;
    rd(x, y);
    writeln(dist[x - 1] + dist[y - 1]);
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
