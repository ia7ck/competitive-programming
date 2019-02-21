void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  auto a = readln.split.to!(int[]);
  auto g = new int[][](n);
  foreach (_; 1 .. n) {
    int u, v;
    rd(u, v);
    g[u - 1] ~= v - 1;
    g[v - 1] ~= u - 1;
  }
  auto num_r = new int[](n), num_b = new int[](n);
  void f(int i, int p) {
    if (a[i] == 1) {
      num_r[i] = 1;
    } else if (a[i] == 2) {
      num_b[i] = 1;
    }
    foreach (j; g[i]) {
      if (j != p) {
        f(j, i);
        num_r[i] += num_r[j];
        num_b[i] += num_b[j];
      }
    }
  }

  f(0, -1);
  int bad = 0;
  void f2(int i, int p) {
    foreach (j; g[i]) {
      if (j != p) {
        if (num_r[j] > 0 && num_b[j]) {
          bad++;
        } else if (num_r[0] - num_r[j] > 0 && num_b[0] - num_b[j] > 0) {
          bad++;
        }
        f2(j, i);
      }
    }
  }

  f2(0, -1);
  writeln(n - 1 - bad);
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
