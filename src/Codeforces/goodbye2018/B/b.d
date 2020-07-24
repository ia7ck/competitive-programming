void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n;
  rd(n);
  struct P {
    int y, x;
  }

  auto xs = new int[](n), ys = new int[](n);
  bool[P] seen;
  foreach (i; 0 .. n) {
    rd(xs[i], ys[i]);
    if (i > 0) {
      seen[P(xs[i], ys[i])] = true;
    }
  }
  auto vy = new int[](n), vx = new int[](n);
  foreach (i; 0 .. n) {
    rd(vx[i], vy[i]);
  }
  foreach (i; 0 .. n) {
    auto tx = xs[0] + vx[i], ty = ys[0] + vy[i];
    foreach (j; 0 .. n) {
      if (i != j) {
        auto p = P(tx - vx[j], ty - vy[j]);
        if ((p in seen) == null || !seen[p]) {
          goto hell;
        }
        seen[p] = false;
      }
    }
    writeln(tx, " ", ty);
    return;
  hell:
    // revert
    foreach (j; 1 .. n) {
      seen[P(xs[j], ys[j])] = true;
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
