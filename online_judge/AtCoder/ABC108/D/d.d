void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int L;
  rd(L);
  struct E {
    int from, to, cost;
  }

  E[] edges;
  int f(int K) {
    if (K == 2) {
      edges ~= E(1, 2, 0);
      edges ~= E(1, 2, 1);
      return 2;
    }
    if (K & 1) {
      auto last = f(K - 1);
      edges ~= E(1, last, K - 1);
      return last;
    } else {
      auto last = f(K / 2);
      foreach (ref e; edges) {
        e.cost *= 2;
      }
      edges ~= E(last, last + 1, 0);
      edges ~= E(last, last + 1, 1);
      return last + 1;
    }
  }

  auto n = f(L);
  writeln(n, " ", edges.length);
  foreach (e; edges) {
    writeln(e.from, " ", e.to, " ", e.cost);
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
