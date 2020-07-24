void main() {
  import std.stdio, std.string, std.conv, std.algorithm;

  int n, r;
  rd(n, r);
  auto as = readln.split.to!(int[]);

  auto bs = new int[](n);
  foreach (int i, a; as) {
    if (a) {
      foreach (int j; max(0, i - r + 1) .. min(n, i + r)) {
        bs[j]++;
      }
    }
  }
  if (bs.find(0).length > 0) {
    writeln(-1);
    return;
  }
  int num = reduce!"a+b"(as);
  foreach (int i, b; bs) {
    if (as[i]) {
      if (b >= 2) {
        bool ok = true;
        foreach (int j; max(0, i - r + 1) .. min(n, i + r)) {
          ok &= bs[j] >= 2;
        }
        if (ok) {
          num--;
          foreach (int j; max(0, i - r + 1) .. min(n, i + r)) {
            bs[j]--;
          }
        }
      }
    }
  }
  writeln(num);
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
