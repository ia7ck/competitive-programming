void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, m;
  rd(n, m);
  struct E {
    int u, v;
  }

  auto edges = new E[](m);
  foreach (i; 0 .. m) {
    rd(edges[i].u, edges[i].v);
    edges[i].u--;
    edges[i].v--;
  }

  int b = 0;
  foreach (i; 0 .. m) {
    auto g = new int[][](n);
    foreach (j; 0 .. m) {
      if (i != j) {
        g[edges[j].u] ~= edges[j].v;
        g[edges[j].v] ~= edges[j].u;
      }
    }
    auto vis = new bool[](n);
    void f(int i) {
      vis[i] = true;
      foreach (j; g[i]) {
        if (!vis[j]) {
          f(j);
        }
      }
    }

    f(0);
    if (!reduce!((res, v) => (res && v))(true, vis)) {
      b++;
    }
  }
  writeln(b);
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
